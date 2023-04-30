# parside

[![parside](https://github.com/WebOfTrust/parside/actions/workflows/test.yml/badge.svg)](https://github.com/WebOfTrust/parside/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/WebOfTrust/parside/branch/main/graph/badge.svg?token=L8K7H1XXQS)](https://codecov.io/gh/WebOfTrust/parside)

Parser library for Composable Event Streaming Representation (CESR).

This library is **currently under construction**.

## Example

Example parsing code (top level ACDC, KEL and TEL parsing):

```
    fn verify_acdc(&self, creder: &Creder, quadlets: &AttachedMaterialQuadlets, deep: Option<bool>) -> Result<bool> {
        if creder.status()?.is_none() {
            return err!(Error::Validation);
        };

        let vcid = creder.said()?;
        let schema = creder.schema()?;
        let prov = creder.chains()?;

        let event = self.store.get_latest_transaction_event(&vcid)?;
        let state = Serder::new_with_raw(event.as_bytes())?;
        let dtnow = chrono::Utc::now();
        let dte = chrono::DateTime::parse_from_rfc3339(&state.ked()["dt"].to_string()?)?.with_timezone(&chrono::Utc);
        if (dtnow - dte).num_seconds() > Self::DEFAULT_CREDENTIAL_EXPIRY_SECONDS {
            return err!(Error::Validation);
        }

        // added brv here for safety even though unimplemented
        if [Ilkage::rev, Ilkage::brv].contains(&state.ked()[Ids::t].to_string()?.as_str()) {
            return err!(Error::Validation);
        }

        schema_cache().verify(&schema, std::str::from_utf8(&creder.raw())?)?;

        let mut rooted = false;

        for group in quadlets.value() {
            match group {
                CesrGroup::SadPathSigVariant { value } => {
                    for sad_path_sig in value.value() {
                        if sad_path_sig.pather.bext()? != "-" {
                            continue;
                        }
           
                        rooted = true;
           
                        let event = self.store.get_key_event(&sad_path_sig.prefixer.qb64()?, sad_path_sig.seqner.sn()? as u32)?;
                        let serder = Serder::new_with_raw(event.as_bytes())?;
                        if serder.said()? != sad_path_sig.saider.qb64()? {
                            return err!(Error::Verification)
                        }
           
                        let verfers = serder.verfers()?;
                        let mut sigers = vec![];
                        for controller_idx_sig in sad_path_sig.sigers.value() {
                            let siger = &controller_idx_sig.siger;
                            if !sigers.contains(siger) {
                                sigers.push(siger.clone())
                            }
                        }
           
                        let mut verified_indices = vec![];
                        for siger in sigers {
                            if siger.index() as usize > verfers.len() {
                                return err!(Error::Verification);
                            }
           
                            if verfers[siger.index() as usize].verify(&siger.raw(), &creder.raw())? {
                                verified_indices.push(siger.index());
                            }
                        }
           
                        if let Some(tholder) = serder.tholder()? {
                            if !tholder.satisfy(&verified_indices)? {
                                return err!(Error::Verification);
                            }
                        } else {
                            return err!(Error::Verification);
                        }
                    }                        
                },
                _ => return err!(Error::Decoding)
            }
        }

        if !rooted {
            return err!(Error::Verification);
        }

        let edges = if prov.to_map().is_ok() {
            vec![prov]
        } else if prov.to_vec().is_ok() {
            prov.to_vec()?
        } else {
            return err!(Error::Verification);
        };

        for edge in &edges {
            for (label, node) in edge.to_map()? {
                if [Ids::d, "o"].contains(&label.as_str()) {
                    continue;
                }

                let node_said = &node["n"].to_string()?;
                let message = self.store.get_acdc(&node_said)?;

                // here we need to default to true
                if deep.unwrap_or(true) {
                    let pacdc = Creder::new_with_raw(message.as_bytes())?;

                    let (_, message_list) = MessageList::from_stream_bytes(message[pacdc.raw().len()..].as_bytes())?;
                    let group = message_list.messages[0].cesr_group()?;

                    match group {
                        CesrGroup::AttachedMaterialQuadletsVariant { value } => {
                            self.verify_acdc(&pacdc, value, deep)?;
                        },
                        _ => return err!(Error::Decoding)
                    }
                }
            }
        }

        let result = self.store.get_acdc(&vcid);
        let existing = result.is_ok();
        if existing {
            let message = result.unwrap();
            let eacdc = Creder::new_with_raw(message.as_bytes())?;

            // this seems very bad, it means something is in the database that shouldn't be there. how did it get there?
            if vcid != eacdc.said()? {
                return err!(Error::Programmer);
            }
        }

        println!("succesfully verified acdc {vcid}");

        Ok(existing)
    }

    fn verify_key_event(&self, serder: &Serder, quadlets: &AttachedMaterialQuadlets, deep: Option<bool>) -> Result<bool> {
        let pre = serder.pre()?;
        let ked = serder.ked();

        // see if code is supported
        let prefixer = Prefixer::new_with_qb64(&pre)?;

        let sn = serder.sn()?;
        let ilk = ked["t"].to_string()?;
        let said = serder.said()?;

        let mut existing = false;

        let (verfers, tholder) = if serder.est()? {
            let tholder = if let Some(tholder) = serder.tholder()? {
                tholder
            } else {
                return err!(Error::Decoding);
            };

            (serder.verfers()?, tholder)
        } else {
            let (raw, _) = self.store.get_latest_establishment_event_as_of_sn(&pre, sn as u32 - 1)?;
            let serder = Serder::new_with_raw(raw.as_bytes())?;

            let tholder = if let Some(tholder) = serder.tholder()? {
                tholder
            } else {
                return err!(Error::Decoding);
            };

            (serder.verfers()?, tholder)
        };

        let mut verified_indices = vec![0u32; 0];
        for group in quadlets.value() {
            match group {
                CesrGroup::ControllerIdxSigsVariant { value } => {
                    for controller_idx_sig in value.value() {
                        let siger = &controller_idx_sig.siger;

                        if siger.index() as usize > verfers.len() {
                            return err!(Error::Verification);
                        }
   
                        if verfers[siger.index() as usize].verify(&siger.raw(), &serder.raw())? {
                            verified_indices.push(siger.index());
                        }
                    }
                }
                _ => return err!(Error::Decoding)
            }
        }

        if !tholder.satisfy(&verified_indices)? {
            return err!(Error::Verification);
        }

        let labels = match ilk.as_str() {
            Ilkage::icp => &Self::ICP_LABELS,
            Ilkage::rot => &Self::ROT_LABELS,
            Ilkage::ixn => &Self::IXN_LABELS,
            _ => return err!(Error::Decoding),
        };
        if !Self::verify_ked_labels(&ked, labels)? {
            return err!(Error::Validation);
        }

        let inceptive = &ilk == Ilkage::icp;
        let key_event_count = self.store.count_key_events(&pre)?;
        if inceptive {
            if !prefixer.verify(&serder.ked(), Some(true))? {
                return err!(Error::Verification);
            }

            if sn != 0 {
                // must be 0
                return err!(Error::Decoding);
            }

            if said != serder.pre()? {
                return err!(Error::Verification);
            }

            if key_event_count != 0 {
                existing = true;
            }
        } else {
            if !serder.saider().verify(&serder.ked(), Some(inceptive), Some(true), None, None, None)? {
                return err!(Error::Verification);
            }
   
            if sn < 1 {
                return err!(Error::Validation);
            }

            let sno = key_event_count as u128;

            if sn > sno {
                // escrow here
                return err!(Error::OutOfOrder);
            }

            if sn != sno {
                existing = true;
            }

            // this sn implementation will become a problem at around 4 billion events
            let event = self.store.get_key_event(&pre, sn as u32 - 1)?;
            let pserder = Serder::new_with_raw(event.as_bytes())?;
            if pserder.said()? != serder.ked()["p"].to_string()? {
                return err!(Error::Verification);
            }

            if deep.unwrap_or(false) {
                let (_, message_list) = MessageList::from_stream_bytes(event[pserder.raw().len()..].as_bytes())?;
                let group = message_list.messages[0].cesr_group()?;

                match group {
                    CesrGroup::AttachedMaterialQuadletsVariant { value } => {
                        self.verify_key_event(&pserder, value, deep)?;
                    },
                    _ => return err!(Error::Decoding)
                }
            }
        }

        if existing {
            let event = self.store.get_key_event(&pre, sn as u32)?;
            let eserder = Serder::new_with_raw(event.as_bytes())?;

            // this seems very bad, it means something is in the database that shouldn't be there. how did it get there?
            if said != eserder.said()? {
                return err!(Error::Programmer);
            }
        }

        println!("succesfully verified key event [{pre}, {said}]");
       
        Ok(existing)
    }

    fn verify_transaction_event(&self, serder: &Serder, seal_source_couples: &SealSourceCouples, deep: Option<bool>) -> Result<bool> {
        let pre = serder.pre()?;
        let ked = serder.ked();

        // see if code is supported
        let prefixer = Prefixer::new_with_qb64(&pre)?;

        let sn = serder.sn()?;
        let ilk = ked["t"].to_string()?;
        let said = serder.said()?;
        let ri = Self::extract_registry_from_serder(&ilk, &serder)?;

        let mut existing = false;

        if seal_source_couples.value.len() != 1 {
            return err!(Error::Decoding);
        }
        let source_saider = &seal_source_couples.value()[0].saider;
        let source_seqner = &seal_source_couples.value()[0].seqner;

        let labels = match ilk.as_str() {
            Ilkage::vcp => &Self::VCP_LABELS,
            Ilkage::iss => &Self::ISS_LABELS,
            Ilkage::rev => &Self::REV_LABELS,
            _ => return err!(Error::Decoding),
        };
        if !Self::verify_ked_labels(&ked, labels)? {
            return err!(Error::Validation);
        }

        let inceptive = &ilk == Ilkage::vcp || &ilk == Ilkage::iss;
        let transaction_event_count = self.store.count_transaction_events(&pre)?;

        let apre = match ilk.as_str() {
            Ilkage::vcp => {
                if !prefixer.verify(&ked, Some(true))? {
                    return err!(Error::Verification);
                }

                ked["ii"].to_string()?
            },
            Ilkage::iss | Ilkage::rev => {
                if !serder.saider().verify(&ked, Some(false), Some(true), None, None, None)? {
                    return err!(Error::Verification);
                }

                let rievent = self.store.get_transaction_event(&ri, 0)?;
                let riserder = Serder::new_with_raw(rievent.as_bytes())?;

                riserder.ked()["ii"].to_string()?
            },
            _ => return err!(Error::Decoding)
        };

        if !self.verify_anchor(&apre, source_seqner, source_saider, serder, deep)? {
            return err!(Error::Verification);
        }

        if inceptive {
            if sn != 0 {
                // must be 0
                return err!(Error::Decoding);
            }

            if transaction_event_count != 0 {
                existing = true;
            }
        } else {
            if sn < 1 {
                return err!(Error::Validation);
            }

            let sno = transaction_event_count as u128;

            if sn > sno {
                // escrow here
                return err!(Error::OutOfOrder);
            }

            if sn != sno {
                existing = true;
            }

            // this sn implementation will become a problem at around 4 billion events
            let event = self.store.get_transaction_event(&pre, sn as u32 - 1)?;
            let pserder = Serder::new_with_raw(event.as_bytes())?;

            if deep.unwrap_or(false) {
                let (_, message_list) = MessageList::from_stream_bytes(event[pserder.raw().len()..].as_bytes())?;
                let group = message_list.messages[0].cesr_group()?;

                match group {
                    CesrGroup::SealSourceCouplesVariant { value } => {
                        self.verify_transaction_event(&pserder, value, deep)?;
                    },
                    _ => return err!(Error::Decoding)
                }
            }

            if pserder.said()? != serder.ked()["p"].to_string()? {
                return err!(Error::Verification);
            }
        }

        if existing {
            let event = self.store.get_transaction_event(&pre, sn as u32)?;
            let eserder = Serder::new_with_raw(event.as_bytes())?;

            // this seems very bad, it means something is in the database that shouldn't be there. how did it get there?
            if said != eserder.said()? {
                return err!(Error::Programmer);
            }
        }

        println!("succesfully verified transaction event [{pre}, {said}]");

        Ok(existing)
    }

    pub fn ingest_messages(&mut self, messages: &str, deep: Option<bool>) -> Result<()> {
        let (_, message_list) = MessageList::from_stream_bytes(messages.as_bytes())?;
        let mut messages = message_list.messages.iter();

        loop {
            let sadder = messages.next();
            if let Some(sadder) = sadder {
                let payload = sadder.payload()?;
                let raw_string = payload.value.to_string();
                let raw_message = raw_string.as_bytes();
                let result = cesride::common::sniff(raw_message)?;

                if result.ident == Identage::KERI {
                    let serder = Serder::new_with_raw(raw_message)?;

                    let message = messages.next();
                    if let Some(message) = message {
                        let group = message.cesr_group()?;
                        match serder.ked()["t"].to_string()?.as_str() {
                            Ilkage::icp | Ilkage::rot | Ilkage::ixn => {                
                                match group {
                                    CesrGroup::AttachedMaterialQuadletsVariant { value } => {
                                        let existing = self.verify_key_event(&serder, value, deep)?;
                                        if !existing {
                                            let event = String::from_utf8(serder.raw())? + &group.qb64()?;
                                            self.store.insert_key_event(&serder.pre()?, &event)?;
                                        }
                                    },
                                    _ => return err!(Error::Decoding), // we only accept pipelined input at present
                                }
                            },
                            Ilkage::vcp | Ilkage::iss | Ilkage::rev => {
                                match group {
                                    CesrGroup::SealSourceCouplesVariant { value } => {
                                        let existing = self.verify_transaction_event(&serder, value, deep)?;
                                        if !existing {
                                            let event = String::from_utf8(serder.raw())? + &group.qb64()?;
                                            self.store.insert_transaction_event(&serder.pre()?, &event)?;
                                        }
                                    },
                                    _ => return err!(Error::Decoding),
                                }
                            }
                            _ => return err!(Error::Decoding),
                        }
                    } else {
                        return err!(Error::Decoding);
                    }
                } else if result.ident == Identage::ACDC {
                    let creder = Creder::new_with_raw(raw_message)?;

                    let message = messages.next();
                    if let Some(message) = message {
                        let group = message.cesr_group()?;
                        match group {
                            CesrGroup::AttachedMaterialQuadletsVariant { value } => {
                                let existing = self.verify_acdc(&creder, value, deep)?;
                                if !existing {
                                    let acdc = String::from_utf8(creder.raw())? + &group.qb64()?;
                                    self.store.insert_acdc(&creder.said()?, &acdc)?;
                                }
                            },
                            _ => return err!(Error::Decoding), // we only accept pipelined input at present
                        };
                    } else {
                        return err!(Error::Decoding);
                    }
                } else {
                    return err!(Error::Decoding)
                }
            } else {
                break;
            }
        }

        self.encrypt_and_cache_storage()
    }
```

## Community

Parside work currently resides alongside the [cesride](https://github.com/WebOfTrust/cesride) work.

## Bi-weekly Meeting
- [Zoom Link](https://us06web.zoom.us/j/88102305873?pwd=Wm01TEJKUWc0aE51a0QzZ2hNbTV2Zz09)
- [HackMD Link](https://hackmd.io/UQaEI0w8Thy_xRF7oYX03Q?view) Bi-Weekly Meeting Agenda and Minutes
- Slack https://join.slack.com/t/keriworld/shared_invite/zt-14326yxue-p7P~GEmAZ65luGSZvbgFAQ
    - `#cesr` channel.

# Important Reference Material
- WebOfTrust/[ietf-cesr](https://github.com/WebOfTrust/ietf-cesr) repository - IETF draft specification for CESR
- Design Assumptions, Use Cases, and ToDo list - [HackMD link](https://hackmd.io/W2Z39cuSSTmD2TovVLvAPg?view)
- Introductory articles:
    - [#1 CESR Proof Signatures](https://medium.com/happy-blockchains/cesr-proof-signatures-are-the-segwit-of-authentic-data-in-keri-e891c83e070a)
    - [#2 CESR Overview](https://medium.com/happy-blockchains/cesr-one-of-sam-smiths-inventions-is-as-controversial-as-genius-d757f36b88f8)