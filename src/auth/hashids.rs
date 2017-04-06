use std::fmt;
use std::error::Error;

const DEFAULT_ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
const DEFAULT_SEPARATORS: &'static str = "cfhistuCFHISTU";
const SEPARATOR_DIV: f32 = 3.5;
const GUARD_DIV: f32 = 12.0;
const MIN_ALPHABET_LENGTH: usize = 8;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HashIdsError {
    InvalidAlphabetLength
}

pub struct HashIds {
    salt: Box<str>,
    alphabet: Box<[char]>,
    separators: Box<[char]>,
    min_hash_length: usize,
    guards: Box<[char]>,
}

impl fmt::Display for HashIdsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for HashIdsError {
    fn description(&self) -> &str {
        "Invalid alphabet length"
    }
}

impl HashIds {
    
    pub fn new(salt: String) -> Result<HashIds, HashIdsError> {
        HashIds::with_min_length(salt, 0)
    }

    pub fn with_min_length(salt: String, min_hash_length: usize)
        -> Result<HashIds, HashIdsError>
    {
        HashIds::with_min_length_and_alphabet(salt, min_hash_length, DEFAULT_ALPHABET)
    }

    pub fn with_min_length_and_alphabet(salt: String, min_length: usize, alphabet: &str)
        -> Result<HashIds, HashIdsError>
    {
        let mut alphabet = {
            let mut unique_alphabet = Vec::with_capacity(alphabet.len());
            for ch in alphabet.chars() {
                if !unique_alphabet.contains(&ch) {
                    unique_alphabet.push(ch);
                }
            }
            unique_alphabet
        };

        if alphabet.len() < MIN_ALPHABET_LENGTH {
            return Err(HashIdsError::InvalidAlphabetLength);
        }

        let mut separators: Vec<char> = DEFAULT_SEPARATORS.chars().collect();
        for i in (0 .. separators.len()).rev() {
            match alphabet.iter().position(|&c| c == separators[i]) {
                Some(idx) => { alphabet.remove(idx); },
                None => { separators.remove(i); },
            }
        }

        HashIds::shuffle(&mut separators, &salt);

        if separators.is_empty() || (alphabet.len() as f32 / separators.len() as f32) > SEPARATOR_DIV {
            let mut seps_len = ((alphabet.len() as f32) / SEPARATOR_DIV).ceil() as usize;
            if seps_len == 1 {
                seps_len += 1;
            }

            if seps_len > separators.len() {
                let diff = seps_len - separators.len();
                separators.extend_from_slice(&alphabet[..diff]);
                alphabet.drain(..diff);
            } else {
                separators.truncate(seps_len);
            }
        }

        HashIds::shuffle(&mut alphabet, &salt);
        let guard_count = (alphabet.len() as f32 / GUARD_DIV).ceil() as usize;

        let guards;

        match alphabet.len() {
           0...3 => {
               guards = separators[..guard_count].to_vec();
               separators.drain(..guard_count);
           },
           _ => {
               guards = alphabet[..guard_count].to_vec();
               alphabet.drain(..guard_count);
           }
        }

        Ok(HashIds {
            salt: salt.into_boxed_str(),
            min_hash_length: min_length,
            guards: guards.into_boxed_slice(),
            separators: separators.into_boxed_slice(),
            alphabet: alphabet.into_boxed_slice(),
        })
    }

    /// Encode hex
    pub fn encode_hex(&self, hex: &str) -> Option<String> {
        let mut ns: Vec<u64> = Vec::with_capacity(hex.len() / 12);

        for chunk in hex.as_bytes().chunks(12) {
            let mut n = 1;
            for &ch in chunk {
                let digit = match ch {
                    b'0'...b'9' => ch - b'0',
                    b'a'...b'f' => ch - b'a' + 10,
                    b'A'...b'F' => ch - b'A' + 10,
                    _ => return None,
                } as u64;
                n <<= 4;
                n |= digit;
            }
            ns.push(n);
        }
        Some(self.encode(&ns))
    }

    /// Decode hex
    pub fn decode_hex(&self, hash: &str) -> Option<String> {
        use std::fmt::Write;
        match self.decode(hash) {
            Some(numbers) => {
                let mut res = String::new();
                let mut buf = String::new();
                for n in numbers {
                    write!(buf, "{:x}", n).unwrap();
                    res.push_str(&buf[1..]);
                    buf.clear();
                }
                Some(res)
            },
            None => None,
        }
    }

    /// Decode
    pub fn decode(&self, hash: &str) -> Option<Vec<u64>> {
        let mut hash_chars: Vec<char> = hash.chars().collect();

        if let Some(end_guard) = hash_chars.iter().rposition(|c| self.guards.contains(c)) {
            hash_chars.truncate(end_guard);
        }
        if let Some(start_guard) = hash_chars.iter().position(|c| self.guards.contains(c)) {
            hash_chars.drain(..start_guard);
        }
        if hash_chars.iter().any(|c| self.guards.contains(c)) {
            return None;
        }
        if hash_chars.is_empty() {
            return None;
        }

        let number_results = hash_chars.iter().filter(|c| self.separators.contains(c)).count() + 1;
        let mut res = Vec::with_capacity(number_results);

        let lol = hash_chars.remove(0);
        let mut alphabet = self.alphabet.clone();
        let mut buf = String::with_capacity(alphabet.len());
        buf.push(lol);
        buf.push_str(&self.salt);

        if buf.len() > alphabet.len() {
            buf.truncate(alphabet.len());
        }

        let const_buffer_len = buf.len();
        for sub_hash in hash_chars.split(|c| self.separators.contains(c)) {
            buf.truncate(const_buffer_len);
            
            if buf.len() < alphabet.len() {
                let extra_needed = alphabet.len() - buf.len();
                buf.extend(alphabet[..extra_needed].iter());
            }

            HashIds::shuffle(&mut alphabet, &buf);

            if let Some(number) = HashIds::unhash(sub_hash, &alphabet) {
                res.push(number);
            } else {
                return None;
            }
        }
        
        Some(res)
    }

    pub fn encode(&self, ns: &[u64]) -> String {
        if ns.len() == 0 {
            panic!("Unable to encode an empty slice of numbers");
        }

        self._encode(ns)
    }

    fn _encode(&self, ns: &[u64]) -> String {
        let mut number_hash_int = 0;
        for (i, &n) in ns.iter().enumerate() {
            number_hash_int += n % (i as u64 + 100);
        }

        let loli = (number_hash_int % self.alphabet.len() as u64) as usize;
        let lol = self.alphabet[loli];
        let mut res: Vec<char> = Vec::with_capacity(self.min_hash_length);
        res.push(lol);

        let mut alphabet = self.alphabet.clone();
        let mut buf = String::with_capacity(alphabet.len());
        buf.push(lol);
        buf.push_str(&self.salt);

        if buf.len() > alphabet.len() {
            buf.truncate(alphabet.len());
        }

        let const_buf_len = buf.len();
        for (i, &n) in ns.iter().enumerate() {
            buf.truncate(const_buf_len);
            // Don't bother adding anything from alphabet if buffer is long enough
            if buf.len() < alphabet.len() {
                let extra_needed = alphabet.len() - buf.len();
                buf.extend(alphabet[..extra_needed].iter());
            }

            HashIds::shuffle(&mut alphabet, &buf);
            let last = HashIds::hash(n, &alphabet);

            res.extend_from_slice(&last);

            if (i + 1) < ns.len() {
                let mut sep_idx = (n % (last[0] as u64 + 1 as u64)) as usize;
                sep_idx %= self.separators.len();
                res.push(self.separators[sep_idx]);
            }
        }

        if res.len() < self.min_hash_length {
            let guard_idx = ((number_hash_int + res[0] as u64) % self.guards.len() as u64) as usize;
            let guard = self.guards[guard_idx];
            res.insert(0, guard);

            if res.len() < self.min_hash_length {
                let guard_idx = ((number_hash_int + res[2] as u64) % self.guards.len() as u64) as usize;
                let guard = self.guards[guard_idx];
                res.push(guard);
            }
        }

        let half_len = alphabet.len() / 2;
        while res.len() < self.min_hash_length {
            buf.clear();
            buf.extend(alphabet.iter());
            HashIds::shuffle(&mut alphabet, &buf);

            res = alphabet[half_len..]
                .iter()
                .cloned()
                .chain(res.into_iter())
                .chain(alphabet[..half_len].iter().cloned())
                .collect();

            let excess = res.len() - self.min_hash_length;
            if excess > 0 {
                let start_pos = excess / 2;
                res.truncate(start_pos + self.min_hash_length);
                res.drain(..start_pos);
            }
        }

        res.into_iter().collect()
    }

    /// Unhash
    fn unhash(input: &[char], alphabet: &[char]) -> Option<u64> {
        let mut n: u64 = 0;
        for (i, ch) in input.iter().enumerate() {
            if let Some(pos) = alphabet.iter().position(|x| x == ch) {
                n += pos as u64 * (alphabet.len() as u64).pow(input.len() as u32 - i as u32 - 1);
            } else {
                return None;
            }
        }
        Some(n)
    }

    /// Hash
    fn hash(mut input: u64, alphabet: &[char]) -> Vec<char> {
        let len = alphabet.len() as u64;
        let mut res = Vec::new();

        loop {
            let i = (input % len) as usize;
            let ch = alphabet[i];
            res.push(ch);
            input /= len;

            if input == 0 {
                break;
            }
        }

        res.reverse();
        res
    }
    /// Shuffle HashIds from the salt given
    fn shuffle(alphabet: &mut [char], salt: &str) {
        let salt = salt.as_bytes();
        
        // if there is no salt given
        if salt.len() <= 0 { return; }

        let len = alphabet.len();

        let mut i: usize = len - 1;
        let mut v: usize = 0;
        let mut p: usize = 0;

        while i > 0 {
            v %= salt.len();
            let t = salt[v] as usize;
            p += t;
            let j = (t + v + p) % i;

            alphabet.swap(i, j);

            i = i - 1;
            v = v + 1;
        }
    }
}

