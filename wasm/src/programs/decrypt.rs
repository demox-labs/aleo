// // Copyright (C) 2019-2023 Aleo Systems Inc.
// // This file is part of the Aleo library.

// // The Aleo library is free software: you can redistribute it and/or modify
// // it under the terms of the GNU General Public License as published by
// // the Free Software Foundation, either version 3 of the License, or
// // (at your option) any later version.

// // The Aleo library is distributed in the hope that it will be useful,
// // but WITHOUT ANY WARRANTY; without even the implied warranty of
// // MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// // GNU General Public License for more details.

// // You should have received a copy of the GNU General Public License
// // along with the Aleo library. If not, see <https://www.gnu.org/licenses/>.

// use std::str::FromStr;

// use crate::{
//   account::ViewKey,
//   types::{
//     TransitionNative,
//     CurrentNetwork, ProgramIDNative, CiphertextNative, IdentifierNative
//   }
// };
// use snarkvm_console::{prelude::Network, types::{U16, Field, Group}, program::ToBits};
// use snarkvm_synthesizer::{Input, Output};

// use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub struct DecryptTransition {}

// #[wasm_bindgen]
// impl DecryptTransition {
//   pub fn owns_transition(
//     view_key: ViewKey,
//     tpk_str: &str,
//     tcm_str: &str,
//   ) -> Result<bool, String> {
//     console_error_panic_hook::set_once();

//     let tpk = Group::<CurrentNetwork>::from_str(tpk_str)
//       .map_err(|_| "Could not deserialize transition public key".to_string())?;

//     let tcm = Field::<CurrentNetwork>::from_str(tcm_str)
//       .map_err(|_| "Could not deserialize transition commitment".to_string())?;

//     let scalar = *view_key;
//     let tvk = (tpk * *scalar).to_x_coordinate();

//     let tcm_derived = CurrentNetwork::hash_psd2(&[tvk])
//       .map_err(|_| "Could not deserialize transition".to_string())?;

//     return Ok(tcm == tcm_derived);
//   }

//   pub fn decrypt_ciphertext(
//     view_key: ViewKey,
//     ciphertext_str: &str,
//     tpk_str: &str,
//     program_id: &str,
//     function_name_str: &str,
//     index: usize,
//   ) -> Result<String, String> {
//     console_error_panic_hook::set_once();

//     let tpk = Group::<CurrentNetwork>::from_str(tpk_str)
//       .map_err(|_| "Could not deserialize transition public key".to_string())?;

//     let program_id = ProgramIDNative::from_str(program_id)
//       .map_err(|_| "Could not deserialize program id".to_string())?;

//     let function_name = IdentifierNative::from_str(function_name_str)
//       .map_err(|_| "Could not deserialize function name".to_string())?;

//     let scalar = *view_key;
//     let tvk = (tpk * *scalar).to_x_coordinate();

//     let function_id = CurrentNetwork::hash_bhp1024(
//       &(U16::<CurrentNetwork>::new(CurrentNetwork::ID), program_id.name(), program_id.network(), function_name).to_bits_le(),
//     ).map_err(|_| "Could not create function id".to_string())?;

//     let index_field = Field::from_u16(u16::try_from(index).unwrap());
//     let ciphertext_view_key = CurrentNetwork::hash_psd4(&[function_id, tvk, index_field])
//       .map_err(|_| "Could not create ciphertext view key".to_string())?;

//     let ciphertext = CiphertextNative::from_str(ciphertext_str)
//       .map_err(|_| "Could not deserialize ciphertext".to_string())?;

//     let plaintext = ciphertext.decrypt_symmetric(ciphertext_view_key)
//       .map_err(|_| "Could not decrypt ciphertext".to_string())?;

//     Ok(plaintext.to_string())
//   }

//   pub fn decrypt_transition(
//     view_key: ViewKey,
//     transition_str: &str
//   ) -> Result<String, String> {
//     console_error_panic_hook::set_once();

//     let transition: TransitionNative = serde_json::from_str(transition_str)
//       .map_err(|_| "Could not deserialize transition".to_string())?;

//     let scalar = *view_key;
//     let tvk = (*transition.tpk() * *scalar).to_x_coordinate();

//     let function_id = CurrentNetwork::hash_bhp1024(
//       &(U16::<CurrentNetwork>::new(CurrentNetwork::ID), transition.program_id().name(), transition.program_id().network(), transition.function_name()).to_bits_le(),
//     ).map_err(|_| "Could not create function id".to_string())?;

//     let mut decrypted_inputs: Vec<Input<CurrentNetwork>> = vec![]; 
//     let mut decrypted_outputs: Vec<Output<CurrentNetwork>> = vec![];

//     for (index, input) in transition.inputs().iter().enumerate() {
//       if let Input::Private(id, ciphertext_option) = input {
//         if let Some(ciphertext) = ciphertext_option {
//           let index_field = Field::from_u16(u16::try_from(index).unwrap());
//           let input_view_key = CurrentNetwork::hash_psd4(&[function_id, tvk, index_field])
//             .map_err(|_| "Could not create input view key".to_string())?;
//           let plaintext = ciphertext.decrypt_symmetric(input_view_key).unwrap();
//           decrypted_inputs.push(Input::Public(*id, Some(plaintext)));
//         } else {
//           decrypted_inputs.push(input.clone());
//         }
//       } else {
//           decrypted_inputs.push(input.clone());
//       }
//     }

//     let num_inputs = transition.inputs().len();
//     for (index, output) in transition.outputs().iter().enumerate() {
//       if let Output::Private(id, ciphertext_option) = output {
//         if let Some(ciphertext) = ciphertext_option {
//           let index_field = Field::from_u16(u16::try_from(num_inputs + index).unwrap());
//           let output_view_key = CurrentNetwork::hash_psd4(&[function_id, tvk, index_field])
//             .map_err(|_| "Could not create output view key".to_string())?;
//           let plaintext = ciphertext.decrypt_symmetric(output_view_key).unwrap();
//           decrypted_outputs.push(Output::Public(*id, Some(plaintext)));
//         } else {
//           decrypted_outputs.push(output.clone());
//         }
//       } else {
//           decrypted_outputs.push(output.clone());
//       }
//     }

//     let decrypted_transition = TransitionNative::new(
//       *transition.program_id(),
//       *transition.function_name(),
//       decrypted_inputs,
//       decrypted_outputs,
//       transition.finalize().cloned(),
//       transition.proof().clone(),
//       *transition.tpk(),
//       *transition.tcm()
//     ).unwrap();

//     let transition_output = serde_json::to_string(&decrypted_transition)
//         .map_err(|_| "Could not serialize decrypted transition".to_string())?;

//     Ok(transition_output)
//   }
// }

// #[cfg(test)]
// mod tests {
//     use std::str::FromStr;

//     use super::*;
//     use wasm_bindgen_test::*;

//     const VIEW_KEY: &str = "AViewKey1nPQW8P83ajkMBHQwYjbUfjGHVSkBQ5wctpJJmQvW1SyZ";
//     const INCORRECT_VIEW_KEY: &str = "AViewKey1o8Hqq4tVbVMeeGtkEGUR7ULghFN8j89sqNQKYRZfe21u";
//     const TRANSITION_VIEW_KEY: &str = "AViewKey1mSnpFFC8Mj4fXbK5YiWgZ3mjiV8CxA79bYNa8ymUpTrw";
//     const TRANSITION: &str = r#"
//       {
//         "id": "as1pe954nkrsz4ztq7tphfug0cxtk4t0v5nnh885llkxufkckc0pcqq64fdjh",
//         "program": "credits.aleo",
//         "function": "transfer",
//         "inputs": [
//             {
//                 "type": "record",
//                 "id": "7627242362896255517779759121644670167065542804779447008052019369271498021878field",
//                 "tag": "7326825649979738473754819542510294000608550604334299567498630301585328020355field"
//             },
//             {
//                 "type": "private",
//                 "id": "5890350227539634203276798567594921209939645071583932668707733854543695228358field",
//                 "value": "ciphertext1qgqz2ypza9srfjnncjzz3hegltwmk0y348ufmklcuqwep2u9wnsqwqkrgx49dn0x78uqypznmyv8r80zwkte9rkfucv7fk4hw7w5s86dzyjmktp7"
//             },
//             {
//                 "type": "private",
//                 "id": "3523824429192332435402492789955521910058950257573863610460494169456702420796field",
//                 "value": "ciphertext1qyqfuz7006rcq9utzdsthdxqv4ra59u58wuggcacv44ka5uv7gyjcrsfh5xwh"
//             }
//         ],
//         "outputs": [
//             {
//                 "type": "record",
//                 "id": "8375863992930925508608168893083821026035462437311607208725719081756051927038field",
//                 "checksum": "327996249778261324588393772992776501551729208590293775377741829891566277743field",
//                 "value": "record1qyqsqv0te3n9fws54jjywmrp36lc8l5gxgzyc9anjk30qsf7h45nfvgpqyxx66trwfhkxun9v35hguerqqpqzq8hd7es9dptx8l6ldn7u536g3hefvl03e6ztrufgk97ekf0us6azgl65lhfgcm4jf7fua2pcc2asy7r46rzv7eefvc8yrs39sgadue3zkl3emg"
//             },
//             {
//                 "type": "record",
//                 "id": "3831168224702324801478452706121992429555677457517630037556628292830553507758field",
//                 "checksum": "104402939347002555939092140274082734465350067270030368157770539944634402431field",
//                 "value": "record1qyqspsy0qghu8wqmf8wq2w4ccqqg8zsgxc3ge2znf4uklh8tutq2swgqqyxx66trwfhkxun9v35hguerqqpqzq9c6u30j7srax79wdvdqt2ytpne4vyvae6z9fq85rs09nj2f72uqm0kn9tx5t3znnj7hrqffzdcquacgyrqdfuuum2km7wvxcmy258svvkjzsh"
//             }
//         ],
//         "proof": "proof1qqqqzqqqqqqqqqqqjgn70r0h5xcsysua8uve5wk400ash3ry6dwr3jjmeft5dmmql03ju0memtrwzppfsyl9x25v6svgrled6hd4s2887yz6wdde7fmv3kwlrdjx8kpvzq5asy02sljyc87ya7me3h5nkh3davwqklw6m2qzszt850x7jq0kt45zghwah6kalw7ufdh2v83jcrcwcpcwwa0m44sestdagm0z7hqe20zlfszva22kfqgpvnj9mavgqw2v5rmeeyz8hmn2j29npkvgteg0447zh6c097tx4dr2vmu2n5ts67xqu83058sjus3srrdgcypql8mymv7rhg580m5gckc4vnwjm2a53vg9sgqmyahs4fhmm0t0atyp9gjvflpt76d2nnsaqyqntm08rmtg0mwzajy2lpyfgq0r0gwq6pqcraty4623uyrzz8036np4clx3ak54qdlfamprav0chq95d696hsy4sdpsfphxuzq5mmehl0pjgk3f7wuvtjshz9dyrnrcwggnmjdqw965fmnjhlxv86ddruqj3cur9r38g2v4evaf2n5clr0844aek7j2gvz4zgshfddlkrg92wzk4yfwdjrwuvmpv77ss2f3efypelqu8sjp23fk93ygdads9lqtz8ghggdy5uhe9j7cyrg2ug4ghww9vvfljk2rgk04sfm23n8j474gzsmzz0nptrtdqmr2afddp5acssa5twxlcpf6vcghssrdan52wrykz5evryzvarw0xj9y0zf2ddarqxqfv2rcjfey9ur7tmaeh2qvqv8z9ggg8vtajql6vj2vuw5shmxsjahcq2ve7m3m3s8a30vy0qx47u263g77hz448mxug4r99vfgkpggv7rysklv0e9l40nt20uvnkuepeftgqwlz7t436z93fpq5qadxsr2tl93t87czw68h6nsglh9xxnenasa2f68vl7pvqahnjlyatcvzyytqxrglvgax9525hwvn939k9jtxzjeh97chr07qgvsp6f007c3p7hdca6cm7ss7wmdrefehzzpj4rpj30cnu2rhdce35ku3y640avsxlujsxnfs69g32q3nlqe7tlcka9zkmeurxx3fcq054sseehe2kqjr2tfdwmgfzgj28vynw4nxq54pvmpgkj53asfnt25yz250lmx0vzqyqqqqqqqqqqq9c4hem5wef967dqy4spcypsr8kwhnmxp35zlrdgq6rwejyqej2l2h6w2lnc7ttw2qxlj8shfju5czqwrcnaj00ky0yc98jck2rk43upw4gzxk6l866n0mh68q0vjalg0qd7tvlu4an04s3u799u28vct6wwm2gn0r5lpcv5jttds6ffw6ykkq6g42yvlam6zreceeqwqz25mrqqqqqppqwa5",
//         "tpk": "853764860907185272244987221391264508066723405990098126446569313951469774602group",
//         "tcm": "2890501131780933954363007654088502278361631362478553633880506988907453958068field"
//       }
//     "#;

//     #[wasm_bindgen_test]
//     fn test_decrypt_transition() {
//         let view_key = ViewKey::from_str(TRANSITION_VIEW_KEY).unwrap();

//         let decrypted_transition_str = DecryptTransition::decrypt_transition(view_key, TRANSITION).unwrap();

//         let decrypted_transition: TransitionNative = serde_json::from_str(&decrypted_transition_str.clone()).unwrap();
//         let public_input = decrypted_transition.inputs().into_iter().skip(1).next().unwrap();
//         if let Input::Public(_id, plaintext_option) = public_input {
//             let plaintext = plaintext_option.as_ref().unwrap();
//             assert_eq!(plaintext.to_string(), "aleo146dx5e4nssf49t0aq9qljk474kqxk848tl05m8w84vc0jqa30spqf4me04");
//         } else {
//             panic!("Expected public input");
//         }

//         // let public_output = decrypted_transition.outputs().into_iter().next().unwrap();
//         // if let Output::Public(_id, plaintext_option) = public_output {
//         //     let plaintext = plaintext_option.as_ref().unwrap();
//         //     assert_eq!(plaintext.to_string(), "100000000u64");
//         // } else {
//         //     panic!("Expected public output");
//         // }
//     }

//     #[wasm_bindgen_test]
//     fn test_decrypt_ciphertext_input() {
//         let view_key = ViewKey::from_str(VIEW_KEY).unwrap();

//         // Try decrypting private input
//         let plaintext = DecryptTransition::decrypt_ciphertext(
//           view_key,
//           "ciphertext1qyq2786j69kjqmwz7lk9cn3glyq2w34j6zhlvxum6u9xkfk76hmd2rgg34kev",
//           "3681563105640905751787370687361466941855498391730203508101562167054325552256group",
//           "helloworld.aleo",
//           "main",
//           1
//         ).unwrap();

//         assert_eq!(plaintext, "2u32");
//     }

//     #[wasm_bindgen_test]
//     fn test_decrypt_ciphertext_output() {
//         let view_key = ViewKey::from_str(VIEW_KEY).unwrap();

//         // Try decrypting private output
//         let plaintext = DecryptTransition::decrypt_ciphertext(
//           view_key,
//           "ciphertext1qyqw68078jwlvz6v2wynue3g3dndyv0ydqutlmn99sfashquhkf52zql6xu7r",
//           "3681563105640905751787370687361466941855498391730203508101562167054325552256group",
//           "helloworld.aleo",
//           "main",
//           2
//         ).unwrap();

//         assert_eq!(plaintext, "3u32");
//     }

//     #[wasm_bindgen_test]
//     fn test_owns_transition_true() {
//         let view_key = ViewKey::from_str(VIEW_KEY).unwrap();

//         let owns_transition = DecryptTransition::owns_transition(
//           view_key,
//           "3681563105640905751787370687361466941855498391730203508101562167054325552256group",
//           "3205548165782039452146864733009325261935114902820697593223360259711032449007field"
//         ).unwrap();
        
//         assert!(owns_transition);
//     }

//     #[wasm_bindgen_test]
//     fn test_owns_transition_false() {
//         let view_key = ViewKey::from_str(INCORRECT_VIEW_KEY).unwrap();

//         let owns_transition = DecryptTransition::owns_transition(
//           view_key,
//           "3681563105640905751787370687361466941855498391730203508101562167054325552256group",
//           "3205548165782039452146864733009325261935114902820697593223360259711032449007field"
//         ).unwrap();
        
//         assert!(!owns_transition);
//     }
// }