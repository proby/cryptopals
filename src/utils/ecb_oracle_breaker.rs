use super::{encryption_oracle::EcbOracle, oracle_tools};

pub struct EcbOracleBreaker {
    oracle: EcbOracle,
    oracle_results: Vec<Vec<u8>>,
    block_size: usize,
    total_blocks_needed: usize,
}

impl EcbOracleBreaker {
    pub fn break_it(oracle: EcbOracle) -> Vec<u8> {
        let mut ecb_oracle_breaker = EcbOracleBreaker {
            oracle,
            oracle_results: vec![],
            block_size: 0,
            total_blocks_needed: 0,
        };

        ecb_oracle_breaker.initialize();
        ecb_oracle_breaker.determine_all_blocks()
    }

    fn initialize(&mut self) {
        // Discover the block size of the cipher.
        self.set_block_size();

        // Detect and ensure that the function is using ECB.
        self.ensure_aes_ecb();

        // Determine total blocks needed
        self.set_total_blocks_needed();

        // Build the set of basic initial oracle resutls so we don't continually recalculate them
        self.set_oracle_results_for_block();
    }

    fn determine_all_blocks(&self) -> Vec<u8> {
        let capacity = self.block_size * self.total_blocks_needed;
        (0..self.total_blocks_needed).fold(
            Vec::with_capacity(capacity),
            |mut total_result, block_index| {
                let mut block_result = self.determine_block(block_index, &total_result);
                total_result.append(&mut block_result);
                total_result
            },
        )
    }

    fn determine_block(&self, block_index: usize, previous_block_results: &[u8]) -> Vec<u8> {
        let block_offset = self.block_size * (block_index + 1);

        let mut results: Vec<u8> = Vec::with_capacity(self.block_size);
        let mut prefix_bytes = vec![0; self.block_size - 1];
        prefix_bytes.append(&mut previous_block_results.to_owned());

        for iter in 0..self.block_size {
            let oracle_response = self.oracle_results.get(iter).expect("asdf");
            let oracle_response_target = oracle_response.get(0..block_offset).expect("oops");

            prefix_bytes.push(0);
            let result = self
                .determine_last_byte(
                    block_offset,
                    prefix_bytes.to_owned(),
                    oracle_response_target,
                )
                .expect("Failed to determine last byte");

            prefix_bytes.remove(0);
            if let Some(last) = prefix_bytes.last_mut() {
                *last = result;
            }

            // if the byte results equals the number of bytes left to determine, we're done
            // AKA undoing the pkcs padding
            if usize::from(result) == (block_offset - prefix_bytes.len()) {
                break;
            }
            results.push(result);
        }

        results
    }

    // cheater const to make this go faster
    const BYTES_TO_TRY: &'static [u8; 37] = &[
        32, 111, 105, 110, 121, 116, 115, 97, 108, 10, 104, 114, 100, 109, 103, 119, 98, 101, 117,
        112, 118, 106, 82, 39, 53, 46, 48, 87, 45, 99, 84, 68, 63, 78, 44, 73, 1,
    ];
    fn determine_last_byte(
        &self,
        block_offset: usize,
        mut prefix_bytes: Vec<u8>,
        target: &[u8],
    ) -> Option<u8> {
        for last_byte in EcbOracleBreaker::BYTES_TO_TRY {
            // used to be "for last_byte in 0..127"
            prefix_bytes[block_offset - 1] = *last_byte;
            let oracle_response = self.oracle.encrypt(&prefix_bytes);
            let blocks = oracle_response.get(0..block_offset).expect("oops");
            if blocks == target {
                return Some(*last_byte);
            }
        }

        None
    }

    fn set_oracle_results_for_block(&mut self) {
        let mut bytes_to_send = vec![0; self.block_size];
        for _ in 0..self.block_size {
            bytes_to_send.pop();
            let oracle_response = self.oracle.encrypt(&bytes_to_send);
            self.oracle_results.push(oracle_response);
        }
    }

    fn set_block_size(&mut self) {
        let mut prefixed_bytes: Vec<u8> = vec![];
        let blank_ciphertext_len = self.oracle.encrypt(&prefixed_bytes).len();

        loop {
            prefixed_bytes.push(0);
            let ciphertext = self.oracle.encrypt(&prefixed_bytes);
            if ciphertext.len() > blank_ciphertext_len {
                self.block_size = ciphertext.len() - blank_ciphertext_len;
                return;
            }
        }
    }

    fn set_total_blocks_needed(&mut self) {
        let ciphertext = self.oracle.encrypt(b"");
        self.total_blocks_needed = ciphertext.len() / self.block_size;
    }

    fn ensure_aes_ecb(&self) {
        let ciphertext = self.oracle.encrypt(b"asdfasdfasdfasdfasdfasdfasdfasdfasdf");
        let mode = oracle_tools::detect_mode(&ciphertext);
        assert_eq!(mode, "ECB");
    }
}
