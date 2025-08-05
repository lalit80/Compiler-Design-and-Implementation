%a = 10
%b = 15
%c = %a + %b
%d = %c + %a
%e = %d - 1

X10 X12 ... X20      ->     Available registers

---------------------------------------------------------
trait Set<T> {
    

}
callee:
    STR     FP, [SP, #-8]!
    STR     LR, [SP, #-8]!
    MOV     FP, SP
    STR     X10 to X20              // or as least as possible required 
    .
    .
    LDR     X10 to X20
    ret

---------------------------------------------------------

aggr Variable_Info
    var_name : String,
    fp : u64,                           // [fp - 8], [fp - 16], ...
    reg : i8,                           // -1 == in memory, else register assigned
    line_no : DArray<usize>,            // store in sorted order on which line the variable was used 
rgga

enum Side
    LHS, RHS
mune

decl var var_info : DArray<Variable_Info> = DArray::new();
decl var cur_line : usize;                                  
decl var cur_word : String;                                 // in next_word
decl var cur_side : Side;
decl var cur_inst : InstructionType;

sbr variable(s : DArray<char>, d : DArray<char>)
    fill_var_info();                                // call after start of a function

    assign_register();
    
rbs

sbr assign_register()
    cur_line = 1;
    decl var next_avail_reg : u8 = 1;               // next_avail_reg == 0 -> no reg free

    while cur_sym != CParenthesis do
        next_word();
        if check(Variable)
            if cur_side == LHS
                define_var();
            elif cur_side == RHS
                use_var();
            decl var p : isize = find_variable(cur_word);
            if var_info[p].reg == -1 && next_avail_reg == 0
                next_avail_reg = save_some_variable();
                assign_some_register(next_avail_reg);
            elif var_info[p].reg == -1 && next_avail_reg != 0
                assign_some_register(next_avail_reg);
            fi
        elif check(Arrow)
            cur_side = Side::RHS;
        elif check(NewLine)
            cur_side = Side::LHS;
            translate_to_register();
            remove_line_no(cur_line);
            cur_line += 1;
        fi
    od
rbs

sbr remove_line_no(t : usize)
    // from var_info for all variables which contain t in line_no array remove it
rbs

sbr fill_var_info()
    // var_name and all the line it exists on
    cur_line = 1;
    decl var j : u64 = 8;
    
    while cur_sym != CParenthesis do
        next_word();
        if cur_sym == Variable
            decl var p : isize = find_variable(cur_word);
            if p == -1
                decl var t = Variable_Info {var_name : cur_word, fp : j, reg : -1, line_no : DArray::new() };
                t.line_no.push(cur_line);
                var_info.push(t);
                j += 8;
            elif p != -1
                var_info[p].line_no.push(cur_line);
            fi
        elif cur_sym == NewLine
            cur_line += 1;
        fi  
    od
rbs

sbr find_variable(cur_word : String) -> isize
    // returns -1 if cur_word is not present in var_info
    // else return the index at which it is present
rbs



























































aggr Variable_Info
    var_name : String,
    defined_on : DArray<u64>,
    used_on : DArray<u64>,
rgga

aggr DU_Data
    line_no : u64,
    defined_var : Set,
    used_var : Set
rgga

enum Side
    LHS, RHS
mune

decl var var_info : DArray<Variable_Info> = DArray::new();
decl var du_data : DArray<DU_Data> = DArray::new();
decl var cur_line : u64;
decl var cur_side : Side;

sbr variable(s : DArray<char>, d : DArray<char>)
    def_and_use();
    in_and_out();
rbs

sbr in_and_out()
    
rbs

sbr def_and_use()
    cur_line = 1;
    while cur_sym != CParenthesis do
        next_word();
        if check(Variable)
            if cur_side == LHS
                define_var(cur_word, cur_line);
            elif cur_side == RHS
                use_var(cur_word, cur_line);
            fi
        elif check(Arrow)
            cur_side = Side::RHS;
        elif check(NewLine)
            cur_side = Side::LHS;
            cur_line += 1;
        fi
    od
rbs

sbr define_var(v_name : String, line_no : u64)
    decl var k = find_variable(v_name);
    if k != -1
        var_info[k].defined_on.push(line_no);
    else if k == -1
        var_info.push(Variable_Info {var_name = v_name, defined_on = DArray::new(), used_on = DArray::new() });
        var_info[var_info.length() - 1].defined_on.push(line_no);
    fi
    du_data.push(get_new_DU_Data());
rbs

sbr use_var(v_name : String, line_no : u64)
    decl var k = find_variable(v_name);
    if k != -1
        var_info[k].used_on.push(line_no);
    elif k == -1
        error("Should be defined first");
    fi
rbs

sbr find_variable(cur_word : String) -> isize
    // returns -1 if cur_word is not present in var_info
    // else return the index at which it is present
rbs






aggr Set

rgga

impl Set
    sbr noOfElements() -> u64
    rbs
lpmi

sbr union(s1 : Set, s2 : Set) -> Set
rbs

sbr difference(s1 : Set, s2 : Set) -> Set
rbs
