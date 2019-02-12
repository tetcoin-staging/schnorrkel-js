extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate schnorrkel;

mod wrapper;
use wrapper::*;

/// Sign a message
/// 
/// The combination of both public and private key must be provided.
/// This is effectively equivalent to a keypair.
/// 
/// * public: UIntArray with 32 element
/// * private: UIntArray with 64 element
/// * message: Arbitrary length UIntArray
/// 
/// * returned vector is the signature consisting of 64 bytes.
#[wasm_bindgen]
pub fn sign(public: &[u8], private: &[u8], message: &[u8]) -> Vec<u8> {
	__sign(public, private, message).to_vec()
}

/// Verify a message and its corresponding against a public key;
/// 
/// * signature: UIntArray with 64 element
/// * message: Arbitrary length UIntArray
/// * pubkey: UIntArray with 32 element
#[wasm_bindgen]
pub fn verify(signature: &[u8], message: &[u8], pubkey: &[u8]) -> bool {
	__verify(signature, message, pubkey)
}

/// Generate a secret key (aka. private key) from a seed phrase.
/// 
/// * seed: UIntArray with 32 element
/// 
/// returned vector is the private key consisting of 64 bytes.
#[wasm_bindgen]
pub fn secret_from_seed(seed: &[u8]) -> Vec<u8> {
	__secret_from_seed(seed).to_vec()
} 

/// Generate a key pair. .
/// 
/// * seed: UIntArray with 32 element
/// 
/// returned vector is the concatenation of first the private key (64 bytes)
/// followed by the public key (32) bytes.
#[wasm_bindgen]
pub fn keypair_from_seed(seed: &[u8]) -> Vec<u8> {
	__keypair_from_seed(seed).to_vec()
}