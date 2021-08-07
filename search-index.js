var searchIndex = JSON.parse('{\
"tracing_android":{"doc":"Integration layer between <code>tracing</code> and Android logs.","t":[3,3,4,13,13,13,13,3,13,13,13,13,6,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,5,11,11,11,11,11,11,5,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["AndroidLogMakeWriter","AndroidLogWriter","Buffer","Crash","Default","Events","Kernel","Layer","Main","Radio","Security","Stats","Subscriber","System","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone_into","compact","default","drop","eq","flatten_event","flush","fmt","fmt","fmt","fmt","fmt_fields","from","from","from","from","init","init","into","into","into","into","json","layer","make_writer","make_writer_for","new","new","subscriber","to_owned","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","with_buffer","with_buffer","with_current_span","with_span_events","with_span_list","with_target","with_thread_ids","with_thread_names","write"],"q":["tracing_android","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["The writer produced by <code>AndroidLogMakeWriter</code>.","A <code>MakeWriter</code> suitable for writing Android logs.","An Android log buffer.","The crash log buffer.","Let the logging function choose the best log target.","The event log buffer.","The kernel log buffer.","A <code>Layer</code> that writes formatted representations of <code>tracing</code> …","The main log buffer.","The radio log buffer.","The security log buffer.","The statistics log buffer.","A <code>Subscriber</code> that writes formatted representations of …","The system log buffer.","","","","","","","","","","","See <code>tracing_subscriber</code> documentation","","","","See <code>tracing_subscriber</code> documentation","","","","","","See <code>tracing_subscriber</code> documentation","","","","","Creates a <code>Subscriber</code> with the given tag and attempts to …","Converts <code>self</code> into a <code>Subscriber</code> and attempts to set it as …","","","","","See <code>tracing_subscriber</code> documentation","Returns a new formatting layer that can be composed with …","","","Returns a new <code>Layer</code> with the given tag.","Returns a new <code>AndroidLogWriter</code> with the given tag.","Returns a <code>Subscriber</code> by wrapping <code>self</code> in a <code>Registry</code>.","","","","","","","","","","","","","","Returns a new <code>Layer</code> with the given tag and using the …","Returns a new <code>AndroidLogMakeWriter</code> with the given tag and …","See <code>tracing_subscriber</code> documentation","See <code>tracing_subscriber</code> documentation","See <code>tracing_subscriber</code> documentation","See <code>tracing_subscriber</code> documentation    ","See <code>tracing_subscriber</code> documentation","See <code>tracing_subscriber</code> documentation",""],"i":[0,0,0,1,1,1,1,0,1,1,1,1,0,1,2,1,3,4,2,1,3,4,1,1,2,1,3,1,2,3,2,1,3,4,2,2,1,3,4,0,2,2,1,3,4,2,0,4,4,2,4,2,1,2,1,3,4,2,1,3,4,2,1,3,4,2,4,2,2,2,2,2,2,3],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["buffer",4]],[[]],[[],[["compact",3],["layer",3,["compact"]]]],[[]],[[]],[[["buffer",4]],["bool",15]],[[["bool",15]]],[[],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[],["layer",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],[["layer",3,["jsonfields","json"]],["json",3],["jsonfields",3]]],[[],["layer",3]],[[]],[[["metadata",3]]],[[]],[[["string",3]]],[[],["subscriber",6]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[["buffer",4]]],[[["buffer",4],["string",3]]],[[["bool",15]]],[[["fmtspan",3]]],[[["bool",15]]],[[["bool",15]]],[[["bool",15]]],[[["bool",15]]],[[],[["result",6,["usize"]],["usize",15]]]],"p":[[4,"Buffer"],[3,"Layer"],[3,"AndroidLogWriter"],[3,"AndroidLogMakeWriter"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};