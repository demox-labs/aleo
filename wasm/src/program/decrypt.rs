// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Aleo library.

// The Aleo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo library. If not, see <https://www.gnu.org/licenses/>.

use std::str::FromStr;

use crate::{
  account::ViewKey,
  types::{
    TransitionNative,
    CurrentNetwork, ProgramIDNative, CiphertextNative, IdentifierNative
  }
};
use snarkvm_console::{prelude::Network, types::{U16, Field, Group}, program::ToBits};
use snarkvm_synthesizer::{Input, Output};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct DecryptTransition {}

#[wasm_bindgen]
impl DecryptTransition {
  pub fn owns_transition(
    view_key: ViewKey,
    tpk_str: &str,
    tcm_str: &str,
  ) -> Result<bool, String> {
    console_error_panic_hook::set_once();

    let tpk = Group::<CurrentNetwork>::from_str(tpk_str)
      .map_err(|_| "Could not deserialize transition public key".to_string())?;

    let tcm = Field::<CurrentNetwork>::from_str(tcm_str)
      .map_err(|_| "Could not deserialize transition commitment".to_string())?;

    let scalar = *view_key;
    let tvk = (tpk * *scalar).to_x_coordinate();

    let tcm_derived = CurrentNetwork::hash_psd2(&[tvk])
      .map_err(|_| "Could not deserialize transition".to_string())?;

    return Ok(tcm == tcm_derived);
  }

  pub fn decrypt_ciphertext(
    view_key: ViewKey,
    ciphertext_str: &str,
    tpk_str: &str,
    program_id: &str,
    function_name_str: &str,
    index: usize,
  ) -> Result<String, String> {
    console_error_panic_hook::set_once();

    let tpk = Group::<CurrentNetwork>::from_str(tpk_str)
      .map_err(|_| "Could not deserialize transition public key".to_string())?;

    let program_id = ProgramIDNative::from_str(program_id)
      .map_err(|_| "Could not deserialize program id".to_string())?;

    let function_name = IdentifierNative::from_str(function_name_str)
      .map_err(|_| "Could not deserialize function name".to_string())?;

    let scalar = *view_key;
    let tvk = (tpk * *scalar).to_x_coordinate();

    let function_id = CurrentNetwork::hash_bhp1024(
      &(U16::<CurrentNetwork>::new(CurrentNetwork::ID), program_id.name(), program_id.network(), function_name).to_bits_le(),
    ).map_err(|_| "Could not create function id".to_string())?;

    let index_field = Field::from_u16(u16::try_from(index).unwrap());
    let ciphertext_view_key = CurrentNetwork::hash_psd4(&[function_id, tvk, index_field])
      .map_err(|_| "Could not create ciphertext view key".to_string())?;

    let ciphertext = CiphertextNative::from_str(ciphertext_str)
      .map_err(|_| "Could not deserialize ciphertext".to_string())?;

    let plaintext = ciphertext.decrypt_symmetric(ciphertext_view_key)
      .map_err(|_| "Could not decrypt ciphertext".to_string())?;

    Ok(plaintext.to_string())
  }

  pub fn decrypt_transition(
    view_key: ViewKey,
    transition_str: &str
  ) -> Result<String, String> {
    console_error_panic_hook::set_once();

    let transition: TransitionNative = serde_json::from_str(transition_str)
      .map_err(|_| "Could not deserialize transition".to_string())?;

    let scalar = *view_key;
    let tvk = (*transition.tpk() * *scalar).to_x_coordinate();

    let function_id = CurrentNetwork::hash_bhp1024(
      &(U16::<CurrentNetwork>::new(CurrentNetwork::ID), transition.program_id().name(), transition.program_id().network(), transition.function_name()).to_bits_le(),
    ).map_err(|_| "Could not create function id".to_string())?;

    let mut decrypted_inputs: Vec<Input<CurrentNetwork>> = vec![]; 
    let mut decrypted_outputs: Vec<Output<CurrentNetwork>> = vec![];

    for (index, input) in transition.inputs().iter().enumerate() {
      if let Input::Private(id, ciphertext_option) = input {
        if let Some(ciphertext) = ciphertext_option {
          let index_field = Field::from_u16(u16::try_from(index).unwrap());
          let input_view_key = CurrentNetwork::hash_psd4(&[function_id, tvk, index_field])
            .map_err(|_| "Could not create input view key".to_string())?;
          let plaintext = ciphertext.decrypt_symmetric(input_view_key).unwrap();
          decrypted_inputs.push(Input::Public(*id, Some(plaintext)));
        } else {
          decrypted_inputs.push(input.clone());
        }
      } else {
          decrypted_inputs.push(input.clone());
      }
    }

    let num_inputs = transition.inputs().len();
    for (index, output) in transition.outputs().iter().enumerate() {
      if let Output::Private(id, ciphertext_option) = output {
        if let Some(ciphertext) = ciphertext_option {
          let index_field = Field::from_u16(u16::try_from(num_inputs + index).unwrap());
          let output_view_key = CurrentNetwork::hash_psd4(&[function_id, tvk, index_field])
            .map_err(|_| "Could not create output view key".to_string())?;
          let plaintext = ciphertext.decrypt_symmetric(output_view_key).unwrap();
          decrypted_outputs.push(Output::Public(*id, Some(plaintext)));
        } else {
          decrypted_outputs.push(output.clone());
        }
      } else {
          decrypted_outputs.push(output.clone());
      }
    }

    let decrypted_transition = TransitionNative::new(
      *transition.program_id(),
      *transition.function_name(),
      decrypted_inputs,
      decrypted_outputs,
      transition.finalize().cloned(),
      transition.proof().clone(),
      *transition.tpk(),
      *transition.tcm(),
      *transition.fee()
    ).unwrap();

    let transition_output = serde_json::to_string(&decrypted_transition)
        .map_err(|_| "Could not serialize decrypted transition".to_string())?;

    Ok(transition_output)
  }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use wasm_bindgen_test::*;

    const VIEW_KEY: &str = "AViewKey1nPQW8P83ajkMBHQwYjbUfjGHVSkBQ5wctpJJmQvW1SyZ";
    const INCORRECT_VIEW_KEY: &str = "AViewKey1o8Hqq4tVbVMeeGtkEGUR7ULghFN8j89sqNQKYRZfe21u";
    const TRANSITION: &str = r#"
    {
      "id": "as14wzlfvv9cull2jrsr3ztsn7pma2cptffxldwn40ppwaq6a9nqsxqsep6pw",
      "program": "helloworld.aleo",
      "function": "main",
      "inputs": [
        {
          "type": "public",
          "id": "6011850806253879925528904662086575409504776042360709979138138321802660391424field",
          "value": "1u32"
        },
        {
          "type": "private",
          "id": "2983267435334865505205591145372676429237317132312651186349157123706950491874field",
          "value": "ciphertext1qyq2786j69kjqmwz7lk9cn3glyq2w34j6zhlvxum6u9xkfk76hmd2rgg34kev"
        }
      ],
      "outputs": [
        {
          "type": "private",
          "id": "616911454638522943602431921505553025557343212973362710068098238074865924658field",
          "value": "ciphertext1qyqw68078jwlvz6v2wynue3g3dndyv0ydqutlmn99sfashquhkf52zql6xu7r"
        }
      ],
      "proof": "proof1qqqqzqqqqqqqqqqq8ajg20qmcqdnnlrms8pc93w704c93t6st6wmzg4fmgqgwh5h6q9644gde2445rv0lksvc5xznr6qpxpqwlvnqy565lapgxlrpckl48mqzl6jxvr2tg04gpaawyd22sf85xvhe3z8dzc7krqmqm2cqpeqsq9qh5paswwnf0ycxevszcrnw56q3x8r0nms758avhlcnzysn7x6tk2c4dwnlwgz6jkk0svhs6sv3qqpvxu8g9e6ayl4qdatt9krt8tswpcz75pt80xh5swgjel8qqxmltx3tqnd5adcurckfampuz22epkqqf690z93sxv4datr36fwaxawe9dwqpnaeua295gn5up5hd5r0ap3qnct4yec04xtjnu2p7uhpdg3qzknpncvu6wtqa95jug6677velctx84np24n7rws4z7pg8l0mss05cvx2hutx98ehj3uyumht6vffqwx6whpyrk4wzq3th0at2v79rcf5zjdusl2adkxlv2407wzmtuh0d52ezer9y0wezkh5ma9ncutygq2lcqsrmzck9dut0yhpja8lanh84nq2p5e6878dqs3clt8dk2z8q7mk5yarnrm599le26f5l9jercpprph4v0aqznwf7tzj7vg8gve9mtyny5833g56kw7n6ds64cu0766f3kj4lghhq9k8s0snynkt97gr43xqjupu3en833meshkvumdkr76cm93tqspkxfw6v7uve7c97szfrcesuhkqk5fs6y007evv8zmszxsyyn8d76gyp28wk7ch3pvanus32kdgn706u6xjqzphxkh9cws79xkps2nez73sfca4a0v49r9hqyjzq25ajcaryjj7u54dwktk3qvnkdtk7he6k7r6d3fdzxg5hxdh3l3yjv7r8qx4j3jmfm2v60x358phfsnnwgaf5mt45nnxfxvhyp4t2namlqg37d8x9m3jrl9g93g7ydqx7re6mux3vjhlgs8r5szzmfh3mnjkna6y0ge2q57w6f005z3p0ufln0yxwhgtzq5fy54yrssuc7xpldqqzrcahlhhe4ls45zvfgsy9z4umlp8gufh5gujvwm3qe9zfxx4tra87knq8cmxkdync2g4lcyk0rllhy2wh424k82ysjuxeeymc8j4eg2ypznkgsm4ja40mr5kqgqyqqqqqqqqqqq0cc5dnzyqqn3zvz9phzm40u23sxq33zyu03ws2ufmvwk8hxylhht60s0gr2w2r4t5c02j7q9jkrqqq07fj8a8d7wew0esxzhhk9uddun0upnlnz6g28y2r8apus8m94tpvmsrslhr906fefzcnfsndqrnpsqhwc60dqcfqatz6nuc42z37jrcaatawy8frwskl0hs2grrwnrhqqqqqyp88yk",
      "tpk": "3681563105640905751787370687361466941855498391730203508101562167054325552256group",
      "tcm": "3205548165782039452146864733009325261935114902820697593223360259711032449007field",
      "fee": 0
    }
    "#;

    #[wasm_bindgen_test]
    fn test_decrypt_transition() {
        let view_key = ViewKey::from_str(VIEW_KEY).unwrap();

        let decrypted_transition_str = DecryptTransition::decrypt_transition(view_key, TRANSITION).unwrap();

        let decrypted_transition: TransitionNative = serde_json::from_str(&decrypted_transition_str.clone()).unwrap();
        let public_input = decrypted_transition.inputs().into_iter().skip(1).next().unwrap();
        if let Input::Public(_id, plaintext_option) = public_input {
            let plaintext = plaintext_option.as_ref().unwrap();
            assert_eq!(plaintext.to_string(), "2u32");
        } else {
            panic!("Expected public input");
        }

        let public_output = decrypted_transition.outputs().into_iter().next().unwrap();
        if let Output::Public(_id, plaintext_option) = public_output {
            let plaintext = plaintext_option.as_ref().unwrap();
            assert_eq!(plaintext.to_string(), "3u32");
        } else {
            panic!("Expected public output");
        }
    }

    #[wasm_bindgen_test]
    fn test_decrypt_ciphertext_input() {
        let view_key = ViewKey::from_str(VIEW_KEY).unwrap();

        // Try decrypting private input
        let plaintext = DecryptTransition::decrypt_ciphertext(
          view_key,
          "ciphertext1qyq2786j69kjqmwz7lk9cn3glyq2w34j6zhlvxum6u9xkfk76hmd2rgg34kev",
          "3681563105640905751787370687361466941855498391730203508101562167054325552256group",
          "helloworld.aleo",
          "main",
          1
        ).unwrap();

        assert_eq!(plaintext, "2u32");
    }

    #[wasm_bindgen_test]
    fn test_decrypt_ciphertext_output() {
        let view_key = ViewKey::from_str(VIEW_KEY).unwrap();

        // Try decrypting private output
        let plaintext = DecryptTransition::decrypt_ciphertext(
          view_key,
          "ciphertext1qyqw68078jwlvz6v2wynue3g3dndyv0ydqutlmn99sfashquhkf52zql6xu7r",
          "3681563105640905751787370687361466941855498391730203508101562167054325552256group",
          "helloworld.aleo",
          "main",
          2
        ).unwrap();

        assert_eq!(plaintext, "3u32");
    }

    #[wasm_bindgen_test]
    fn test_owns_transition_true() {
        let view_key = ViewKey::from_str(VIEW_KEY).unwrap();

        let owns_transition = DecryptTransition::owns_transition(
          view_key,
          "3681563105640905751787370687361466941855498391730203508101562167054325552256group",
          "3205548165782039452146864733009325261935114902820697593223360259711032449007field"
        ).unwrap();
        
        assert!(owns_transition);
    }

    #[wasm_bindgen_test]
    fn test_owns_transition_false() {
        let view_key = ViewKey::from_str(INCORRECT_VIEW_KEY).unwrap();

        let owns_transition = DecryptTransition::owns_transition(
          view_key,
          "3681563105640905751787370687361466941855498391730203508101562167054325552256group",
          "3205548165782039452146864733009325261935114902820697593223360259711032449007field"
        ).unwrap();
        
        assert!(!owns_transition);
    }
}