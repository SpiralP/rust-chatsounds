window.SIDEBAR_ITEMS = {"enum":[["EventType","SND_SEQ_EVENT_xxx constants"],["QuerySubsType","snd_seq_query_subs_type_t wrapper"]],"struct":[["Addr","snd_seq_addr_t wrapper"],["ClientInfo","snd_seq_client_info_t wrapper"],["ClientIter","Iterates over clients connected to the seq API (both kernel and userspace clients)."],["Connect","snd_seq_connect_t wrapper"],["EvCtrl",""],["EvNote",""],["EvQueueControl","snd_seq_ev_queue_control_t wrapper"],["EvResult","snd_seq_result_t wrapper"],["Event","snd_seq_event_t wrapper"],["Input","Struct for receiving input events from a sequencer. The methods offered by this object may modify the internal input buffer of the sequencer, which must not happen while an `Event` is alive that has been obtained from a call to `event_input` (which takes `Input` by mutable reference for this reason). This is because the event might directly reference the sequencer’s input buffer for variable-length messages (e.g. Sysex)."],["MidiEvent","snd_midi_event_t Wrapper"],["PortCap","SND_SEQ_PORT_CAP_xxx constants"],["PortInfo","snd_seq_port_info_t wrapper"],["PortIter","Iterates over clients connected to the seq API (both kernel and userspace clients)."],["PortSubscribe","snd_seq_port_subscribe_t wrapper"],["PortSubscribeIter","Iterates over port subscriptions for a given client:port/type."],["PortType","SND_SEQ_PORT_TYPE_xxx constants"],["QueueStatus","snd_seq_queue_status_t wrapper"],["QueueTempo","snd_seq_queue_tempo_t wrapper"],["Remove","SND_SEQ_REMOVE_xxx constants"],["RemoveEvents","snd_seq_remove_events_t wrapper"],["Seq","snd_seq_t wrapper"]],"trait":[["EventData","Low level methods to set/get data on an Event. Don’t use these directly, use generic methods on Event instead."]]};