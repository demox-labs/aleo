import init, * as aleo from '@aleohq/wasm';

await init();

await aleo.initThreadPool(navigator.hardwareConcurrency);

let TRANSFER_KEY;
let INCLUSION_KEY;

self.addEventListener("message", ev => {
  // Load Transfer Prover Key
  if (ev.data.type == 'ALEO_LOAD_TRANSFER_KEY') {
    console.log('Web worker: Deserialize Transfer Key...');
    let startTime = performance.now();
    TRANSFER_KEY = aleo.ProvingKey.from_bytes(ev.data.transferProverBytes);
    console.log(`Web worker: Deserialized transfer proving key Completed: ${performance.now() - startTime} ms`);
    self.postMessage({ type: 'TRANSFER_KEY_DESERIALIZED' });
  } 
  // Load Inclusion Prover Key
  else if (ev.data.type == 'ALEO_LOAD_INCLUSION_KEY') {
    console.log('Web worker: Deserialize Inclusion Key...');
    console.log(ev.data);
    let startTime = performance.now();
    INCLUSION_KEY = aleo.ProvingKey.from_bytes(ev.data.inclusionProverBytes);
    console.log(`Web worker: Deserialized inclusion proving key Completed: ${performance.now() - startTime} ms`);
    self.postMessage({ type: 'INCLUSION_KEY_DESERIALIZED' });
  } 
  // Create Transition
  else if (ev.data.type == 'ALEO_CREATE_TRANSITION') {
    const {
      privateKey,
      toAddress,
      amount,
      plaintext
    } = ev.data;
    const program = aleo.Program.credits();

    console.log('Web worker: Building Transition for program: ', program.id());
    let startTime = performance.now();

    // Prepare inputs
    const pK = aleo.PrivateKey.from_string(privateKey);
    const inputs = JSON.stringify([plaintext, toAddress, `${amount}u64`])

    const transition = aleo.TransactionBuilder.build_transition(
      program,
      'transfer',
      inputs,
      pK,
      TRANSFER_KEY
    );
    console.log(`Web worker: Transition Completed: ${performance.now() - startTime} ms`);
    console.log(`Transition: ${transition}`);
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
    let startTime = performance.now();
    let transitionParsed = JSON.parse(transition);
    let inputIdsParsed = JSON.parse(inputIds);

    const transaction = aleo.TransactionBuilder.build_transaction(
      INCLUSION_KEY,
      JSON.stringify([{ transition: transitionParsed, input_ids: inputIdsParsed}]),
      stateRoot,
      statePaths
    );
    console.log(`Web worker: Transaction Completed: ${performance.now() - startTime} ms`);
    console.log(`Transaction: ${transaction}`);
    self.postMessage({ type: 'TRANSACTION_COMPLETED', transaction });
  }
  else if (ev.data.type == 'ALEO_VERIFY_TRANSACTION') {
    const {
      transaction,
      transferVerifierBytes,
      inclusionVerifierBytes
    } = ev.data;
    console.log('Web worker: Verifying Transaction...');
    let startTime = performance.now();

    const program = aleo.Program.credits();
    const functionName = 'transfer';
    const transferVerifyingKey = aleo.VerifyingKey.from_bytes(transferVerifierBytes);
    const inclusionVerifyingKey = aleo.VerifyingKey.from_bytes(inclusionVerifierBytes);

    const verified = aleo.TransactionBuilder.verify_transaction(
      transaction,
      program,
      functionName,
      transferVerifyingKey,
      inclusionVerifyingKey,
      true,
      true
    );
    console.log(`Web worker: Transaction Verified: ${performance.now() - startTime} ms`);
    console.log(verified);
  }
});