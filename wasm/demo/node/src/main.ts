import { ControllerIdxSigs, ControllerIdxSig, Message, Siger, CesrGroup, MessageList } from "parside-wasm";

function main() {
    const messageStream = "-AABAABg3q8uNg1A2jhEAdbKGf-QupQhNnmZQx3zIyPLWBe6qqLT5ynytivf9EwJhxyhy87a0x2cezDdil4SsM2xxs0O"
    const messageListStream = "{\"v\":\"KERI10JSON00012b_\",\"t\":\"icp\"}-CABBD8-gMSJ6K1PQ7_gG5ZJn2NkHQJgdkiNrTBz_FWWS_cC0BDc1i44ZX0jaIHh5oNDx-TITbPnI6VEn2nKlqPwkkTF452X7XxYh80tolDpReYwZpnD8TF4Or2v3CpSCikyt6EG{\"v\":\"KERI20JSON00981b_\",\"t\":\"tcp\"}-AABAABg3q8uNg1A2jhEAdbKGf-QupQhNnmZQx3zIyPLWBe6qqLT5ynytivf9EwJhxyhy87a0x2cezDdil4SsM2xxs0Orest"
    console.log(`--Start--`)
    parseMessage(Buffer.from(messageStream))
    parseMessageList(Buffer.from(messageListStream))

    const newGroup = buildGroup()
    console.log(`--Assert parsed and built groups--`)
    console.assert(messageStream, newGroup.qb64())
    console.log(`--End--`)
}

function parseMessage(bytes: Uint8Array) {
    console.log('--Parse message from bytes--')
    const result = Message.from_stream_bytes(bytes)
    console.log(`   Rest: ${result.rest}`)
    console.log(`   Message: ${result.message}`)
    handleMessage(result.message)
}

function parseMessageList(bytes: Uint8Array) {
    console.log('--Parse message list from bytes--')
    const result = MessageList.from_stream_bytes(bytes)
    console.log(`   Rest: ${result.rest}`)
    console.log(`   Messages: ${result.messages.length}`)
    if (result.messages) {
        for (const message of result.messages) {
            handleMessage(message)
        }
    }
}

function handleMessage(message: Message) {
    if (message.group) {
        iterateOverGroup(message.group)
    }
    if (message.payload) {
        console.log(`Parsed Payload: ${message.payload.toString()}`)
    }
}

function iterateOverGroup(group: CesrGroup) {
    if (group.controllerIdxSigs) handleControllerIdxSigs(group.controllerIdxSigs)
    if (group.firstSeenReplayCouples) console.log('         Handle FirstSeenReplayCouples')
    if (group.nonTransReceipt) console.log('        Handle NonTransReceipt')
    if (group.pathedMaterialQuadlets) console.log('         Handle PathedMaterialQuadlets')
    if (group.sadPathSigGroups) console.log('           Handle SadPathSigGroups')
    if (group.sadPathSigs) console.log('        Handle SadPathSigs')
    if (group.sealSourceCouples) console.log('          Handle SealSourceCouples')
    if (group.transIdxSigGroups) console.log('          Handle TransIdxSigGroups')
    if (group.transLastIdxSigGroups) console.log('          Handle TransLastIdxSigGroups')
    if (group.transReceiptQuadruples) console.log('         Handle TransReceiptQuadruples')
    if (group.witnessIdxSigs) console.log('         Handle WitnessIdxSigs')
}

function handleControllerIdxSigs(controllerIdxSigs: ControllerIdxSigs) {
    console.log('       Handle ControllerIdxSigs')
    console.log('           Iterate over items')
    console.log('               Option 1: for index loop')
    const size = controllerIdxSigs.size() || 0
    for (let i = 0; i < size; i++) {
        const controllerIdxSig = controllerIdxSigs.value(i);
        console.log(`                   Siger: ${controllerIdxSig?.siger.qb64()}`)
    }

    const values = controllerIdxSigs.values() ?? []
    console.log('               Option 2: for of loop')
    for (const controllerIdxSig of values) {
        console.log(`                   Siger: ${controllerIdxSig.siger.qb64()}`)
    }
}

function buildGroup(): ControllerIdxSigs {
    console.log('--Build a group--')
    const siger = Siger.new_with_qb64("AABg3q8uNg1A2jhEAdbKGf-QupQhNnmZQx3zIyPLWBe6qqLT5ynytivf9EwJhxyhy87a0x2cezDdil4SsM2xxs0O")
    const controllerIdxSig = new ControllerIdxSig(siger)
    const controllerIdxSigs = new ControllerIdxSigs([ controllerIdxSig ])
    console.log(`   ControllerIdxSigs: ${controllerIdxSigs.qb64()}`)
    return controllerIdxSigs
}

main()
