import import.aleo;

program parent.aleo;

record who_called_parent:
    owner as address.private;
    gates as u64.private;
    // self.caller, should be original invoker of the program calling this one
    caller as address.private;
    // self.parent, should be the direct program address calling this one
    parent as address.private;

function example:
    // invoked by original caller, parent and caller should be the same
    cast self.caller 0u64 self.caller self.parent into r0 as who_called_parent.record;

    // external_call's parent should be this program, but caller should match the original caller
    call import.aleo/external_call into r1;

    // invoked by original caller, parent and caller should be the same
    cast self.caller 0u64 self.caller self.parent into r2 as who_called_parent.record;

    // external_call's parent should be this program, but caller should match the original caller
    call import.aleo/external_call into r3;

    output r0 as who_called_parent.record;
    output r1 as import.aleo/who_called_import.record;
    output r2 as who_called_parent.record;
    output r3 as import.aleo/who_called_import.record;
