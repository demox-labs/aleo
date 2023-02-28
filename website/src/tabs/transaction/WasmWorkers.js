import React, {useState, useEffect} from "react";
import {Card, Divider, Form, Input, Button } from "antd";
const { TextArea } = Input;
import {useAleoWASM} from "../../aleo-wasm-hook";
import {downloadAndStoreFiles, getSavedFile} from '../../db';
import init, * as aleo from '@aleohq/wasm';

await init();

export const WasmWorkers = () => {
    const aleo = useAleoWASM();
    const layout = {labelCol: {span: 4}, wrapperCol: {span: 21}};

    useEffect(async () => {
        await init();
        aleo.start_up_worker();
    });

    if (aleo !== null) {
        return <Card title="Wasm Worker" style={{width: "100%", borderRadius: "20px"}} bordered={false}>
            <Form {...layout}>
                <Form.Item label="Number" colon={false}>
                    <Input id="inputNumber" name="number" size="large" placeholder="Input a number" allowClear
                          style={{borderRadius: '20px'}}/>
                </Form.Item>
            </Form>
        </Card>
    } else {
        return <h3>
            <center>Loading...</center>
        </h3>
    }
}