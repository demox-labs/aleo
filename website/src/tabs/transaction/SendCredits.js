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
    // const [privateKey, setPrivateKey] = useState("APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH"); // AViewKey1mSnpFFC8Mj4fXbK5YiWgZ3mjiV8CxA79bYNa8ymUpTrw
    
    // AViewKey1e9dHqEbCGmh9wXzirbeoB1GAmfaAdGdW9e49X9oNDBMZ  aleo1mc6h9k9g03hfkrh93wm6vlew6q2ehzwalqutzjx6hp5mdcv9rsyq2w3alz
    const [privateKey, setPrivateKey] = useState("APrivateKey1zkpDKDJELtg3GN29R2UPihgsucvYSkb7BzEYDRkJtZcwhj8");
    // APrivateKey1zkpCGWDkqECBaj14Bcmn7cVPP6LtWHKcihQ5GESSNHEcGyu AViewKey1fj1WFBBeqQZ5q5ytK9p7SS9G6QDYBmmaND2BfVpUMq1J
    const [toAddress, setToAddress] = useState("aleo1nhdttzqjjj68qqjge68m7ugpz6d9dq30qxxngyg2qjr9f8hx4cgs42t7wp");
    const [amount, setAmount] = useState(9973);
    const [plaintext, setPlaintext] = useState(`{  owner: aleo1mc6h9k9g03hfkrh93wm6vlew6q2ehzwalqutzjx6hp5mdcv9rsyq2w3alz.private,  gates: 1000000u64.private,  _nonce: 1533294440985855610247015716364689691059517427540009237071243460486400881193group.public}`);
    // const [stateRoot, setStateRoot] = useState("ar1r3jmesfh0umg8fax9xruk9peqhgyamdecxklp5wfd2sp8ptt3y8sznca24");
    // const [statePaths, setStatePaths] = useState(`{ "map":{ "4808019121071627648091077876462306045081749577842248810791756201298887594188field": "path1qqqxcgpg0qnqdq7j8kcqh3wcc2wpul7dan6z9jydknlx2zhe85kdvzgqqqqqqqqqqqqfevh64sx84vguwa4af94fpr9dyxjhkpa49x33j0cvfuy24gje5ydn7xfdsu5d60atwq2vwqvq95s46e9fy26zmuzzlc4dchd0h993z9szr7zx9fv07srrk6e78p96vz2zfhnzdydjm0q2lzepl6h6nnhq4r32xjcvr40x3z9fe5d8qrh3sqtkmdhutjj4gmmad2zlrd5cfdq9exwe4r5204pmys8e7qsz7ajdvgewledhvh6dkpzqyvcv5zv9dyxa5w8ag0886az3ft9wkx3yh555aa7w92k5w03exwrndmvnal9wsrffgd4muv9h4rz293c2pjrzk0jxykq5tq0ryxsn3hlwyky9yyhxqdk5cvfqvphkp25zrh07dazegch7nj29ggru0m46vdzeep9sjkxsqnugz3p6mxvaz8zkmcxqdrrcwkh7turqqe5a34dpej63gymzpwcqss2eafg236dqunyghdd0p7pttvey0765ry52vccqmlp859j9tgzgg9v7559gaxswfjytkkhslq44kvj8ld2pj29xvvqdlsn6zez45pyyzk022z5wng8yez9mttc0s26mxfrlk4qe9znxxqxlcfapv326qjzpt849p28f5rjv3za44u8c9ddny3lm2svj3fnrqr0uy7skg4dqfpq4n6js4r56pexg3w667ruzkkejgla4gxfg5e3sph7z0gty2ksyss2eafg236dqunyghdd0p7pttvey0765ry52vccqmlp859j9tgzgg9v7559gaxswfjytkkhslq44kvj8ld2pj29xvvqdlsn6zez45pyyzk022z5wng8yez9mttc0s26mxfrlk4qe9znxxqxlcfapv326qjzpt849p28f5rjv3za44u8c9ddny3lm2svj3fnrqr0uy7skg4dqfpq4n6js4r56pexg3w667ruzkkejgla4gxfg5e3sph7z0gty2ksyss2eafg236dqunyghdd0p7pttvey0765ry52vccqmlp859j9tgzgg9v7559gaxswfjytkkhslq44kvj8ld2pj29xvvqdlsn6zez45pyyzk022z5wng8yez9mttc0s26mxfrlk4qe9znxxqxlcfapv326qjzpt849p28f5rjv3za44u8c9ddny3lm2svj3fnrqr0uy7skg4dqfpq4n6js4r56pexg3w667ruzkkejgla4gxfg5e3sph7z0gty2ksyss2eafg236dqunyghdd0p7pttvey0765ry52vccqmlp859j9tgzgg9v7559gaxswfjytkkhslq44kvj8ld2pj29xvvqdlsn6zez45pyyzk022z5wng8yez9mttc0s26mxfrlk4qe9znxxqxlcfapv326qjzpt849p28f5rjv3za44u8c9ddny3lm2svj3fnrqr0uy7skg4dqfpq4n6js4r56pexg3w667ruzkkejgla4gxfg5e3sph7z0gty2ksyss2eafg236dqunyghdd0p7pttvey0765ry52vccqmlp859j9tgzgg9v7559gaxswfjytkkhslq44kvj8ld2pj29xvvqdlsn6zez45pyyzk022z5wng8yez9mttc0s26mxfrlk4qe9znxxqxlcfapv326qjgsfxt5xyexxsvl2p7h0v42gmvwstmce28kz0sk9rdhmhfav39pqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqevelxwelpcduuy3g9q3a5qv4pc45xt993ftfgzvru93kvv94fgfqzqqqqqqqqqqqcu6khsj6x34vgqshx5taxe7st0xakvu9c7jl4lwzyfxtpuak0sqx8ce6fm5jhhfup3as6eg7m95h7hhd38f2m2e8w582lwzj253rkp8yg5kuy6dutdpymvjmlecd3we7uyg5rzev0v2dj4pp3xzedmq9pcq3yjr0kwu0thhtggvza0p4sljfhlzxlzsfey3cnln4w7u42c54wqqqqqqqqqqqqqqgg9v7559gaxswfjytkkhslq44kvj8ld2pj29xvvqdlsn6zez45pyyzk022z5wng8yez9mttc0s26mxfrlk4qe9znxxqxlcfapv326qjzpt849p28f5rjv3za44u8c9ddny3lm2svj3fnrqr0uy7skg4dqfpq4n6js4r56pexg3w667ruzkkejgla4gxfg5e3sph7z0gty2ksyss2eafg236dqunyghdd0p7pttvey0765ry52vccqmlp859j9tgzgg9v7559gaxswfjytkkhslq44kvj8ld2pj29xvvqdlsn6zez45pyyzk022z5wng8yez9mttc0s26mxfrlk4qe9znxxqxlcfapv326qjzpt849p28f5rjv3za44u8c9ddny3lm2svj3fnrqr0uy7skg4dqfpq4n6js4r56pexg3w667ruzkkejgla4gxfg5e3sph7z0gty2ksyss2eafg236dqunyghdd0p7pttvey0765ry52vccqmlp859j9tgzgg9v7559gaxswfjytkkhslq44kvj8ld2pj29xvvqdlsn6zez45pyyzk022z5wng8yez9mttc0s26mxfrlk4qe9znxxqxlcfapv326qjzpt849p28f5rjv3za44u8c9ddny3lm2svj3fnrqr0uy7skg4dqfpq4n6js4r56pexg3w667ruzkkejgla4gxfg5e3sph7z0gty2ksyss2eafg236dqunyghdd0p7pttvey0765ry52vccqmlp859j9tgzgg9v7559gaxswfjytkkhslq44kvj8ld2pj29xvvqdlsn6zez45pyazgrvp2y4wurjzzmuyahgqacemtamsp257eynqm73d6s3c9xwpuqqqqqqqqqqqqyyzk022z5wng8yez9mttc0s26mxfrlk4qe9znxxqxlcfapv326qjzpt849p28f5rjv3za44u8c9ddny3lm2svj3fnrqr0uy7skg4dqfpq4n6js4r56pexg3w667ruzkkejgla4gxfg5e3sph7z0gty2ksyss2eafg236dqunyghdd0p7pttvey0765ry52vccqmlp859j9tgzqzqqq9fzwlaqteqmz56k6ug68dzmljtdd30pfft0ugnaj5pkaqkqkqgrsyqqqqqqqqqqqss2eafg236dqunyghdd0p7pttvey0765ry52vccqmlp859j9tgzxmwc6jaymmvvtcn5plj8aayh8azuap7nz2y6u30ey6ln435cxuz5yzk022z5wng8yez9mttc0s26mxfrlk4qe9znxxqxlcfapv326qjzpt849p28f5rjv3za44u8c9ddny3lm2svj3fnrqr0uy7skg4dqgqqzqvqvc5q7pwf69ucrek7r2rsuydz30lth3s6uy6qj66u5kzlhpsl2zzsv0g978" } }`);
    const [transition, setTransition] = useState(`{"id":"as1zlhx59ec4h645u5x2jg52yw2w8wjq2y978c7kp6zp62qjmzcw5gss0pwx6","program":"credits.aleo","function":"transfer","inputs":[{"type":"record","id":"2661860267515426683305195179296562104503705222318905041280227334359114321647field","tag":"5064560508888649720175535400292125470716350016007495219148141431264478118348field"},{"type":"private","id":"1898787011987296073277184819804432952024437265466447436878906214692410879668field","value":"ciphertext1qgqvsr88ureznxvausr06za83kplhufme6qrpfgye3pa24yvl0u2yqkp56lmxuh28pvrmr2ay6u0g5nhpll8ccefwew5h6cjhu5yz3nups4dkr4w"},{"type":"private","id":"5546479586503170421990239372742978604361200344524721081287597222929545287176field","value":"ciphertext1qyqgfhr0dnl3ealgxqpy8qwss76aj8ge52y5f804zls4hc6x7f0pwpq5skn2v"}],"outputs":[{"type":"record","id":"1699636717546963319440354720069172798224117745695424220132567978496134424026field","checksum":"1568028686392006680385562152250254727067972316652730422907800954020682010977field","value":"record1qyqsppwn09lw7rls7d0nkdmnkw0qh7l2a6fwhez72uwdesmhv0vw76ctqyqsq6gg7ewpcaknmncwjes5gm5s5p83r0cks9se5unp6pa5qxfdy0s2qz8nryv3mtfm6jvutlna6ldx55h7gv3waukshu9xnxccnd4smseqcyvd83f"},{"type":"record","id":"7526439256965505806197106062120504258500291032519532431618933031470230183053field","checksum":"860539464123221802195253680052707572153441166171204092488261042134120871883field","value":"record1qyqsqcx909e9qnlkqc7w8rex6gsxvzcdvmap56c4zmqqeygd7qnmycqrqyqsqhujkhtp4lnpfm7deeagf5ezhrzy984xykq3em5c2kkqyeqqj6sqqqng6n0w4w8dtmc46w84s56fu8yys9c898nglr7ujecf7nsjtd9q76455e9"}],"proof":"proof1qqqqzqqqqqqqqqqq4kyqmdyrfhtfcxhtq3syn3at55j8vyq0amxjtuzym983k4zkxchrm3grl575jhx4mjt33uv3vuysptx7kxjk2qnqnh384we5g2tfcecyke3y4h83wt7gzh925ftep8jhr5z2vh0v4pkgkqlf2u5fpfqlqz2uw7f4l5pv9s7hctqnl4lm2a3lv4k2cshw2savldalhzy2hxut60dt67r9yf0egnca5verzmm43qgpdxurykerfz9gl53e5la6xp6edm8gertd7kd4xfdku6ae2r26km3zaycsptjhh470nmm89t7v5v2gpe4a6ven98k496v5gkgan24utyl0sxn44l52xm2v6nkh35c6mq5ytu44dlrhjv3xke6m2zsrcmzgsr9mt8gn8llxs4gn6q9ep47p8skmavzpzy4m4hhuvagxzyfykq278m7nj4lx7j59mstsydnyp9zyyqvvmsn3yc9kwnd4ptzsq974atd70f8q2h3q25sr4p8sus9ch63lhwy9vw5z2x3p7xfefknkhm0hazqfx8nklxla7n304puz2zp0jl9q7d84yjc4ayc2hqf0859k9ecs6dtacfrksgk6j3npx8kkgfqu2x5peu60q4kf9w9kj5jn2hwr564t37835ew3kpjymh7ma5jd35cduz0wltf4m92ar56gsq5zu3whtpkcr65x68q6uydrww945ygvgprse88apd0t0sl9k4adruckwtmjujesle48vtsnuky8thwfnvp7z078sps8nrxjl96flt8ymj0ug2zktmgdyn3qz62z5hel256tuca47sesstuw0cr08at79aju6l8kfzjkmyz2zggrkd0nzyzv5m5zhz5u0aqtk2qq2tyuzvpyaej4yp9j5sqkm3hsndl9rcer9330a3tcf7ueesr968l25xz2kths2lkkkhpysnap7tj8yfdw3utfcng2346kdxq6qpnennhhv5n2y75akaz47k5tdjvc523vunw0qrnx4vt5mq83rplgp2fvzwuvnne0u5293psuhn3ugnvypgxq3tspyq40p4d0k4eswmfsg0uqu3eg8lud3kd6q2hyvd73v38pclp65398hqagx5afwtl2wyqwdta0fjurn0c030gshpn7s33u6qn82ec86mt5n9a0ngmteya2u5gqyqqqqqqqqqqq2f8sy5x59fzcaycq504htmaadthdlatwszk4d8pl5ppfjhnqczjhc8xg0sscthm0sfqhaqrvkzjqzqwcm034vqu87m37vr7zw26h9xygpyn7tyj0m8u8ucxrtr3xztjwq4uz4eqenxddlljpg6uevacax4phpjcllymshgxdfntgknuc2fexc7c824ttw75rjphhk0hammu4zqqqqqntsdw2","tpk":"368835412895937837309323441008762862401271562588688971503137282123171155355group","tcm":"2629661665322110025372679742389673700684558830010394257677893884076645389667field","fee":0}`);
    const [inputIds, setInputIds] = useState(`[{"type":"record","commitment":"4808019121071627648091077876462306045081749577842248810791756201298887594188field","gamma":"4659263130644638829306038213254281214601713206820227756792215909285982172553group","serial_number":"5589473101423835763677311276371240764458053140913095070251050902189880825606field","tag":"8308497125187564238192512891194478945928129859693211933927828912880156591760field"},{"type":"private","id":"3586507452788591212436989017406869037511802968258952509679436677600340928363field"},{"type":"private","id":"3035160781342889385325366620007393715977792390846204652809533286490958908885field"}]`);
    const [stateRoot, setStateRoot] = useState('');
    const [statePaths, setStatePaths] = useState('');
    // const [transition, setTransition] = useState('');
    // const [inputIds, setInputIds] =useState('');
    const [transaction, setTransaction] = useState(null);
    const aleo = useAleoWASM();

    useEffect(() => {
        worker.addEventListener("message", ev => {
            if (ev.data.type == 'TRANSITION_COMPLETED') {
                const data = JSON.parse(ev.data.transition);
                setTransition(JSON.stringify(data[0]));
                setInputIds(JSON.stringify(data[1]));
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
            console.log(`Fetching Transfer Prover from IndexedDb Completed: ${performance.now() - startTime} ms`);
            await postMessagePromise(worker, {
                type: 'ALEO_LOAD_INCLUSION_KEY',
                inclusionProverBytes: inclusionProver.bytes
            });
    
            // Build transition
            await postMessagePromise(worker, {
                type: 'ALEO_CREATE_TRANSACTION',
                transition,
                inputIds,
                stateRoot: newStateRoot,
                statePaths: newStatePaths
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