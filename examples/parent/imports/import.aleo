program import.aleo;

record who_called_import:
    owner as address.private;
    gates as u64.private;
    // self.caller, should be original invoker of the program calling this one
    caller as address.private;
    // self.parent, should be the direct program address calling this one
    parent as address.private;
   
function external_call:
    cast self.caller 0u64 self.caller self.parent into r0 as who_called_import.record;

    output r0 as who_called_import.record;