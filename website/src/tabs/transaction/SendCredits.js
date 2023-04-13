import React, {useState, useEffect} from "react";
import {Card, Divider, Form, Input, Button } from "antd";
const { TextArea } = Input;
import {useAleoWASM} from "../../aleo-wasm-hook";
import {downloadAndStoreFiles, getSavedFile} from '../../db';
import {getStateRoot, inputIdsToStatePathMap} from './getChainState';

const worker = new Worker("./worker.js");

// Create a Promise-based wrapper around postMessage
function postMessagePromise(worker, message) {
    return new Promise((resolve, reject) => {
        worker.postMessage(message);
        worker.onmessage = event => {
            resolve(event.data);
        };
        worker.onerror = error => {
            reject(error);
        };
    });
}

export const SendCredits = () => {
    // Beacon address

    // AViewKey1e9dHqEbCGmh9wXzirbeoB1GAmfaAdGdW9e49X9oNDBMZ  aleo1mc6h9k9g03hfkrh93wm6vlew6q2ehzwalqutzjx6hp5mdcv9rsyq2w3alz
    const [privateKey, setPrivateKey] = useState("APrivateKey1zkpDKDJELtg3GN29R2UPihgsucvYSkb7BzEYDRkJtZcwhj8");
    // APrivateKey1zkpCGWDkqECBaj14Bcmn7cVPP6LtWHKcihQ5GESSNHEcGyu AViewKey1fj1WFBBeqQZ5q5ytK9p7SS9G6QDYBmmaND2BfVpUMq1J
    const [toAddress, setToAddress] = useState("aleo1nhdttzqjjj68qqjge68m7ugpz6d9dq30qxxngyg2qjr9f8hx4cgs42t7wp");
    const [amount, setAmount] = useState(97);
    const [plaintext, setPlaintext] = useState(`{  owner: aleo1mc6h9k9g03hfkrh93wm6vlew6q2ehzwalqutzjx6hp5mdcv9rsyq2w3alz.private,  gates: 9973u64.private,  _nonce: 1157538218493794217455834413151916695596644156738232294925137030650794330428group.public}`);
    
    const [stateRoot, setStateRoot] = useState('');
    const [statePaths, setStatePaths] = useState('');
    const [transition, setTransition] = useState('');
    const [inputIds, setInputIds] =useState('');
    
    const [transaction, setTransaction] = useState("");
    
    const aleo = useAleoWASM();

    useEffect(() => {
        worker.addEventListener("message", ev => {
            if (ev.data.type == 'TRANSITION_COMPLETED') {
                const data = JSON.parse(ev.data.transition);
                setTransition(JSON.stringify(data.transition));
                setInputIds(JSON.stringify(data.input_ids));
            } else if (ev.data.type == 'TRANSACTION_COMPLETED') {
                setTransaction(ev.data.transaction);
            }
          });
    }, []);

    const safeStateUpdate = (update, event) => {
      try { update(event.target.value) }
      catch (error) { console.error(error)}
    }

    const buildTransition = async () => {
      try {
        // Download files
        let startTime = performance.now();
        await downloadAndStoreFiles();
        console.log(`Download Completed: ${performance.now() - startTime} ms`);
        startTime = performance.now();

        // Get transfer prover from IndexedDB
        const transferProver = await getSavedFile('TransferProver');
        console.log(transferProver);
        console.log(`Fetching Transfer Prover from IndexedDb Completed: ${performance.now() - startTime} ms`);
        await postMessagePromise(worker, {
            type: 'ALEO_LOAD_TRANSFER_KEY',
            transferProverBytes: transferProver.bytes
        });

        // Build transition
        await postMessagePromise(worker, {
            type: 'ALEO_CREATE_TRANSITION',
            privateKey,
            amount,
            toAddress,
            plaintext
        });
      } catch (error) { console.error(error) }
    }

    const buildTransaction = async () => {
        try {
            // Update state root
            const newStateRoot = await getStateRoot();
            setStateRoot(newStateRoot);

            // Update state paths
            const newStatePaths = JSON.stringify(await inputIdsToStatePathMap(inputIds));
            setStatePaths(newStatePaths);

            // Get inclusion prover
            let startTime = performance.now();
            const inclusionProver = await getSavedFile('InclusionProver');
            console.log(inclusionProver);
            console.log(`Fetching Inclusion Prover from IndexedDb Completed: ${performance.now() - startTime} ms`);
            const inclusionProverBytes = inclusionProver.bytes;
            await postMessagePromise(worker, {
                type: 'ALEO_LOAD_INCLUSION_KEY',
                inclusionProverBytes
            });
    
            // Build transaction
            await postMessagePromise(worker, {
                type: 'ALEO_CREATE_TRANSACTION',
                transition,
                inputIds,
                stateRoot: newStateRoot,
                statePaths: newStatePaths
            });
        } catch (error) { console.error(error) }
    }

    const verifyTransaction = async () => {
        try {
            // Get transfer verifier
            let startTime = performance.now();
            const transferVerifier = await getSavedFile('TransferVerifier');
            console.log(transferVerifier);
            console.log(`Fetching Transfer Verifier from IndexedDb Completed: ${performance.now() - startTime} ms`);
            const transferVerifierBytes = transferVerifier.bytes;

            // Get inclusion verifier
            startTime = performance.now();
            const inclusionVerifier = await getSavedFile('InclusionVerifier');
            console.log(inclusionVerifier);
            console.log(`Fetching Inclusion Verifier from IndexedDb Completed: ${performance.now() - startTime} ms`);
            const inclusionVerifierBytes = inclusionVerifier.bytes;
    
            // Verify Transaction
            await postMessagePromise(worker, {
                type: 'ALEO_VERIFY_TRANSACTION',
                transaction,
                transferVerifierBytes,
                inclusionVerifierBytes
            });
        } catch (error) { console.error(error) }
    }


    const layout = {labelCol: {span: 4}, wrapperCol: {span: 21}};

    if (aleo !== null) {
        return <Card title="Send Credits" style={{width: "100%", borderRadius: "20px"}} bordered={false}>
            <Form {...layout}>
                <Form.Item label="Private Key" colon={false}>
                    <Input name="privateKey" size="large" placeholder="Private Key" allowClear value={privateKey} onChange={(evt) => safeStateUpdate(setPrivateKey, evt)}
                          style={{borderRadius: '20px'}}/>
                </Form.Item>
                <Form.Item label="To Address" colon={false}>
                    <Input name="toAddress" size="large" placeholder="To Address" allowClear value={toAddress} onChange={(evt) => safeStateUpdate(setToAddress, evt)}
                          style={{borderRadius: '20px'}}/>
                </Form.Item>
                <Form.Item label="Amount" colon={false}>
                    <Input name="amount" size="large" placeholder="Amount" allowClear value={amount} onChange={(evt) => safeStateUpdate(setAmount, evt)}
                          style={{borderRadius: '20px'}}/>
                </Form.Item>
                <Form.Item label="Record (Plain Text)" colon={false}>
                    <TextArea rows={3} name="recordPlainText" size="large" placeholder="Record (Plain Text)" allowClear value={plaintext} onChange={(evt) => safeStateUpdate(setPlaintext, evt)}
                          style={{borderRadius: '20px'}}/>
                </Form.Item>
                <Button onClick={() => buildTransition()}>
                  Create Transition
                </Button>
            </Form>
            {
                (transition !== null) ?
                    <Form {...layout}>
                        <Divider/>
                        <Form.Item label="Transition" colon={false}>
                            <TextArea rows={3} size="large" placeholder="Transition" allowClear value={transition} onChange={(evt) => safeStateUpdate(setTransition, evt)} />
                        </Form.Item>
                        <Form.Item label="Input Ids" colon={false}>
                            <TextArea rows={2} size="large" placeholder="Input Ids" allowClear value={inputIds} onChange={(evt) => safeStateUpdate(setInputIds, evt)} />
                        </Form.Item>
                        <Form.Item label="State Root" colon={false}>
                        <Input name="stateRoot" size="large" placeholder="State Root" allowClear value={stateRoot} onChange={(evt) => safeStateUpdate(setStateRoot, evt)}
                            style={{borderRadius: '20px'}}/>
                        </Form.Item>
                        <Form.Item label="State Paths" colon={false}>
                            <TextArea rows={3} name="statePaths" size="large" placeholder="State Paths" allowClear value={statePaths} onChange={(evt) => safeStateUpdate(setStatePaths, evt)}
                                style={{borderRadius: '20px'}}/>
                        </Form.Item>
                        <Button onClick={() => buildTransaction()}>
                            Create Transaction
                        </Button>
                    </Form>
                    : null
            }
            {
                (transaction !== null) ?
                    <Form {...layout}>
                        <Divider/>
                        <Form.Item label="Transaction" colon={false}>
                            <TextArea rows={7} size="large" placeholder="Transaction" value={transaction} readOnly={true} />
                        </Form.Item>
                        <Button onClick={() => verifyTransaction()}>
                            Verify Transaction
                        </Button>
                    </Form>
                    : null
            }
        </Card>
    } else {
        return <h3>
            <center>Loading...</center>
        </h3>
    }
}