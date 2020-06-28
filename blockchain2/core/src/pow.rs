use crate::block;
use utils::coder;
use bigint::U256;

const MAX_NONCE: u32 = 0x7FFFFFFF;

pub struct ProofOfWork {
    target: U256,  
}

impl ProofOfWork {
    pub fn new_proof_of_work(bits: u32) -> ProofOfWork {  
        println!("bits: {}", bits);     
        let (mant, expt) = {
            let unshifted_expt = bits >> 24;
            println!("unshifted_expt: {}", unshifted_expt);
            if unshifted_expt <= 3 {
                ((bits & 0xFFFFFF) >> (8 * (3-unshifted_expt as usize)), 0)
            } else {
                (bits & 0xFFFFFF, 8 * ((bits >> 24) - 3))
            }
        };

        if mant > 0x7FFFFF {
            println!("default");

            ProofOfWork {
                target: Default::default(),  
            }   
        } else {
            println!("calcute, mant: {}, expt: {}", mant as u64, expt as usize);

            ProofOfWork {
                target: U256::from(mant as u64) << (expt as usize),  
            }
        }
    }

    fn prepare_data(bc: &mut block::Block, nonce: u32) -> Vec<u8> {
        bc.header.nonce = nonce;
        let data = coder::my_serialize(&(bc.header));
        data
    }

    pub fn run(&self, mut bc: &mut block::Block) {
        let mut nonce: u32 = 0;

        println!("target: {:?}", self.target);

        while nonce <= MAX_NONCE {
            let data = ProofOfWork::prepare_data(&mut bc, nonce);
            let mut hash: [u8; 32] = [0; 32];
            coder::get_hash(&data[..], &mut hash);
            // println!("hash === {:?}", hash);
            
            // let hash = String::from("123456789a");
            println!("len ========= {:?}", hash.len());
            let hash_int = U256::from(hash);
            println!("hash_int === {:?}", hash_int);
            
            if hash_int <= self.target {
                println!("Calcute, hash:  {:?}", hash_int);
                bc.hash = hash.iter().map(|&c| c as char).collect::<String>();
                return;
            }

            nonce += 1;
        } 
        

    }
}