import React, {useState} from "react";
import {Card, Divider, Form, Input, Button } from "antd";
const { TextArea } = Input;
import {useAleoWASM} from "../../aleo-wasm-hook";
import {downloadAndStoreFiles, getAllSavedFiles} from '../../db';

export const SendCredits = () => {
    const [privateKey, setPrivateKey] = useState("APrivateKey1zkp3dQx4WASWYQVWKkq14v3RoQDfY2kbLssUj7iifi1VUQ6");
    const [toAddress, setToAddress] = useState("aleo184tj0fllfuzqzpmw5jt6l2ptx0avhjxh95u9llcr6ypf6fx3hvrsref0ju");
    const [amount, setAmount] = useState(50);
    const [plaintext, setPlaintext] = useState(`{
      owner: aleo184vuwr5u7u0ha5f5k44067dd2uaqewxx6pe5ltha5pv99wvhfqxqv339h4.private,
      gates: 1159017656332810u64.private,
      _nonce: 1635890755607797813652478911794003479783620859881520791852904112255813473142group.public
    }`);
    const [transaction, setTransaction] = useState(null);
    const aleo = useAleoWASM();

    const safeStateUpdate = (update, event) => {
      try { update(event.target.value) }
      catch (error) { console.error(error)}
    }

    const buildTransaction = async () => {
      try {
        let startTime = performance.now();
        await downloadAndStoreFiles();
        console.log(`Download Completed: ${performance.now() - startTime} ms`);
        startTime = performance.now();

        const files = await getAllSavedFiles();
        const transferProver = files.filter(({name}) => name == 'TransferProver')[0];
        console.log(transferProver);
        console.log(`Fetching Transfer Prover from IndexedDb Completed: ${performance.now() - startTime} ms`);
        startTime = performance.now();

        const pK = aleo.PrivateKey.from_string(privateKey);
        const provingKey = aleo.ProvingKey.from_bytes(transferProver.bytes);
        console.log(`Deserialized proving key Completed: ${performance.now() - startTime} ms`);
        startTime = performance.now();
        const add = aleo.Address.from_string(toAddress);
        const rec = aleo.RecordPlaintext.fromString(plaintext);
        const transaction = await aleo.TransactionBuilder.build_transfer_full(pK, provingKey, add, BigInt(amount), rec);
        console.log(`Transaction Completed: ${performance.now() - startTime} ms`);
        console.log(transaction);
        setTransaction(transaction);
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
                    <TextArea rows={5} name="recordPlainText" size="large" placeholder="Record (Plain Text)" allowClear value={plaintext} onChange={(evt) => safeStateUpdate(setPlaintext, evt)}
                          style={{borderRadius: '20px'}}/>
                </Form.Item>
                <Button onClick={() => buildTransaction()}>
                  Create Transaction
                </Button>
            </Form>
            {
                (transaction !== null) ?
                    <Form {...layout}>
                        <Divider/>
                        <Form.Item label="Transaction" colon={false}>
                            <TextArea rows={7} size="large" placeholder="Transaction" value={transaction} readOnly={true} />
                        </Form.Item>
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