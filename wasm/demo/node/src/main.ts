import { ControllerIdxSigs, ControllerIdxSig, Message, Siger } from "parside-wasm";

function main() {
    const bytes2 = Buffer.from("-AABAABg3q8uNg1A2jhEAdbKGf-QupQhNnmZQx3zIyPLWBe6qqLT5ynytivf9EwJhxyhy87a0x2cezDdil4SsM2xxs0O", 'utf8');
    const result = Message.from_stream_bytes(bytes2)
    console.log('rest')
    console.log(result.rest())
    console.log('message')
    console.log(result.message())
    const message = result.message()
    const group = message.group()
    const controllerIdxSigs = group?.controllerIdxSigs()
    const size = controllerIdxSigs?.size() || 0
    console.log('controllerIdxSig')
    for (let i = 0; i < size; i++) {
        const controllerIdxSig = controllerIdxSigs?.value(i);
        console.log('controllerIdxSig')
        const siger = controllerIdxSig?.siger()
        console.log(siger?.qb64())
    }

    const values = controllerIdxSigs?.values() || []
    for (const val of values) {
        console.log('value')
        console.log(val?.siger()?.qb64())
    }

    const siger2 = Siger.new_with_qb64("AABg3q8uNg1A2jhEAdbKGf-QupQhNnmZQx3zIyPLWBe6qqLT5ynytivf9EwJhxyhy87a0x2cezDdil4SsM2xxs0O")
    const controllerIdxSig2 = new ControllerIdxSig(siger2)
    const controllerIdxSigs2 = new ControllerIdxSigs([controllerIdxSig2])
    console.log('controllerIdxSigs2')
    console.log(controllerIdxSigs2)
    console.log(controllerIdxSigs2.qb64())
}

main()
