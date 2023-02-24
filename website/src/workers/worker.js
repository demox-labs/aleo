import "babel-polyfill";

let TRANSFER_KEY;
let INCLUSION_KEY;

import("@aleohq/wasm").then(aleo => {
  self.addEventListener("message", ev => {
    if (ev.data.type == 'ALEO_LOAD_TRANSFER_KEY') {
      console.log('Web worker: Deserialize Transfer Key...');
      console.log(ev.data);
      let startTime = performance.now();
      TRANSFER_KEY = aleo.ProvingKey.from_bytes(ev.data.transferProverBytes);
      console.log(`Web worker: Deserialized transfer proving key Completed: ${performance.now() - startTime} ms`);
      self.postMessage({ type: 'TRANSFER_KEY_DESERIALIZED' });
    } else if (ev.data.type == 'ALEO_LOAD_INCLUSION_KEY') {
      console.log('Web worker: Deserialize Inclusion Key...');
      console.log(ev.data);
      let startTime = performance.now();
      INCLUSION_KEY = aleo.ProvingKey.from_bytes(ev.data.inclusionProverBytes);
      console.log(`Web worker: Deserialized inclusion proving key Completed: ${performance.now() - startTime} ms`);
      self.postMessage({ type: 'INCLUSION_KEY_DESERIALIZED' });
    } else if (ev.data.type == 'ALEO_CREATE_TRANSITION') {
      const {
        privateKey,
        toAddress,
        amount,
        plaintext
      } = ev.data;

      console.log('Web worker: Building Transition...');
      console.log(ev.data, TRANSFER_KEY);
      let startTime = performance.now();

      // Deserialize Private Key, Address, and Record
      const pK = aleo.PrivateKey.from_string(privateKey);
      const address = aleo.Address.from_string(toAddress);
      const rec = aleo.RecordPlaintext.fromString(plaintext);

      const transition = aleo.TransactionBuilder.build_transition(
        pK,
        TRANSFER_KEY,
        address,
        BigInt(amount),
        rec
      );
      console.log(`Web worker: Transition Completed: ${performance.now() - startTime} ms`);
      console.log(transition);
      self.postMessage({ type: 'TRANSITION_COMPLETED', transition });
    }
    else if (ev.data.type == 'ALEO_CREATE_TRANSACTION') {
      const {
        transition,
        inputIds,
        stateRoot,
        statePaths
      } = ev.data;

      console.log('Web worker: Building Transaction...');
      console.log(ev.data, INCLUSION_KEY);
      let startTime = performance.now();

      const transaction = aleo.TransactionBuilder.build_transaction(
        INCLUSION_KEY,
        transition,
        inputIds,
        stateRoot,
        statePaths
      );
      console.log(`Web worker: Transaction Completed: ${performance.now() - startTime} ms`);
      console.log(transaction);
      self.postMessage({ type: 'TRANSACTION_COMPLETED', transaction });
    }
  });
});