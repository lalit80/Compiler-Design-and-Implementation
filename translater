aggr word
    a: usize,                               -- a .. b in s indicates a word
    b: usize
rgga

aggr CHF
rgga

aggr Queue
	-- queue data
rgga

#[derive(PartialEq)]
enum i_type
    Load, Store, Branch_LB, Branch_SBR, Return, Expression_1, Expression_2,
    Label, Function_Start, Function_End, Compare
mune

#[derive(PartialEq)]
enum operation_type
    Add, Mul, Sub, LSL, RSL, Nothing, 
    LESS, GREATER, NOTEQUAL, EQUAL, LESSEQUAL, GREATEREQUAL, 
mune

#[derive(PartialEq)]
enum sym
    S(word), lb(word), M, d8(word), G32(char), G64(char), BO, FP, B, Constant(word),
    Num(word), CI, CO, RET, Newline, OParen, CParen, Arrow, EOF, 
    Dollar, Colon, Hash, Zero,
    Nothing
mune

#[derive(PartialEq)]
enum cmp
    EQUAL, LESS, GREATER
mune

aggr TRData
    s : DArray<char>,
    d : DArray<char>,
    i : usize,                              -- s[0] to s[i] indicates part solved
    j : usize,                              -- s[i] to s[j] indicates a word (s[i,j])
	lb_seen : bool,
    
    cur_sym : sym,                          -- stores the symbol of word looked recently
    sbr_data : SbrInfo,
    asm_data : AsmData,
    
rgga

aggr LabelData
    is_defined : bool,
    label_name : word
rgga

aggr SbrData
    sbr_name : word,
    sbr_defined : bool, 
    labels : DArray<LabelData>,
rgga

aggr SbrInfo
    sbr_data : DArray<SbrData>,
rgga

aggr AsmData
    cur_i_type : i_type,
    operand : Queue,
    o_type : operation_type,
    sbr_name : word,
    s : DArray<char>,
    d : DArray<char>,
    i : usize,                                     -- d[0 .. i - 1] contains translated data
rgga

impl TRData

    /*
        sbr translate(s : &[char], d : &var [char])
            TRData tr_data = new TRData();
            tr_data.initialize();
            tr_data.verify_syntax();
        rbs
    */

    sbr initialize(&self)
        self.cur_sym = None;
        self.i = 0;
        self.j = 0;
        self.sbr_data = SbrInfo::new();
        self.asm_data = AsmData::new();
        self.asm_data.initialize(self.s, self.d);
    rbs

    sbr verify_syntax(&self)

        self.validate(sym::OParen);
        self.next_word();
        self.validate(sym::S);
        self.sbr_data.use_sbr(self.cur_sym);

        self.check_R();                             -- check all sbrs

        self.next_word();
        self.validate(sym::CParen);
        self.next_word();
        self.validate(sym::EOF);

        self.check_all_definitions();               -- check all sbr and label definitions

    rbs

    sbr check_R(&self)
        self.next_word();
        self.validate(sym::OParen);
        self.next_word();
        self.validate(sym::S);
        if !self.sbr_data.define_sbr(self.cur_sym)
            self.error();                                            -- redefinition
        fi
        self.asm_data.set_sbr_name(self.cur_sym);               -- extract word
        self.asm_data.operand.enqueue(self.cur_sym);

        self.next_word();
        self.validate(sym::Num);
        self.asm_data.operand.enqueue(self.cur_sym);
        self.asm_data.set_i_type(i_type::Function_Start);
        self.asm_data.put();

        self.check_I();
        
        self.next_word();
        self.validate(sym::CParen);
        self.next_word();
        self.validate(sym::Newline);
        self.asm_data.set_i_type(i_type::Function_End);
        self.asm_data.put();

        self.check_R();
    rbs

    sbr check_I(&self)

        self.next_word();

        if self.check(sym::M)
            if self.lb_seen
                self.lb_seen = false;
            fi

            self.next_word();
            if !(self.check(sym::G32) || self.check(sym::G64) || self.check(sym::FP))
                self.error();
            fi
            self.asm_data.operand.enqueue(self.cur_sym);
            self.next_word();
            self.validate(sym::Constant);
            self.asm_data.operand.enqueue(self.cur_sym);

            self.next_word();
            self.validate(sym::Arrow);

            self.next_word();
            if self.check(sym::G32) || self.check(sym::G64)
                self.asm_data.operand.enqueue(self.cur_sym);
                self.next_word();
                self.validate(sym::Newline);
                self.asm_data.set_i_type(i_type::Store);
                self.asm_data.put();
                self.check_I();
            elif true
                self.error();
            fi

        elif self.check(sym::RET)
            if self.lb_seen
                self.lb_seen = false;
            fi
            self.next_word();
            self.validate(sym::Newline);
            self.asm_data.set_i_type(i_type::Return);
            self.check_I();

        elif self.check(sym::B)
            if self.lb_seen
                self.lb_seen = false;
            fi
            self.next_word();
            if self.check(sym::S)
                self.sbr_data.use_sbr(self.cur_sym);
                self.asm_data.operand.enqueue(self.cur_sym);
                self.next_word();
                self.validate(sym::Newline);
                self.asm_data.set_i_type(i_type::Branch_SBR);
                self.asm_data.put();
                self.check_I();
            elif self.check(sym::lb)
                self.sbr_data.use_lb(self.cur_sym);
                self.asm_data.operand.enqueue(self.cur_sym);
                self.next_word();
                self.validate(sym::Newline);
                self.asm_data.set_i_type(i_type::Branch_LB);
                self.asm_data.put();
                self.check_I();
            elif true
                self.error();
            fi
        
        elif self.check(sym::G32)
            if self.lb_seen
                self.lb_seen = false;
            fi

            self.asm_data.operand.enqueue(self.cur_sym);
            self.next_word();
            if self.check(sym::Arrow)
                self.next_word();
                if self.check(sym::M)
                    self.next_word();
                    if !(self.check(sym::G32) || self.check(sym::G64) || self.check(sym::FP))
                        self.error();
                    fi
                    self.asm_data.operand.enqueue(self.cur_sym);
                    self.next_word();
                    self.validate(sym::d8);
                    self.asm_data.operand.enqueue(self.cur_sym);
                    self.next_word();
                    self.validate(sym::Newline);
                    self.asm_data.set_i_type(i_type::Load);
                    self.asm_data.put();
                    self.check_I();
                elif self.check(sym::G32)
                    self.asm_data.operand.enqueue(self.cur_sym);
                    self.next_word();
                    self.validate(sym::BO);
                    self.asm_data.operand.enqueue(self.cur_sym);
                    self.next_word();
                    self.validate(sym::Arrow);
                    self.next_word();
                    self.validate(sym::V32);
                    self.asm_data.operand.enqueue(self.cur_sym);
                    self.next_word();
                    self.validate(sym::Newline);
                    self.asm_data.set_i_type(i_type::Expression_2);
                    self.asm_data.put();
                    self.check_I();
                fi
                self.error();
            
            elif self.check(sym::BO)
                self.asm_data.operand.enqueue(self.cur_sym);
                self.next_word();
                self.validate(sym::Arrow);
                self.next_word();
                if self.check(sym::Constant) || self.check(sym::G32)
                    self.asm_data.operand.enqueue(self.cur_sym);
                    self.next_word();
                    self.validate(sym::Newline);
                    self.asm_data.set_i_type(i_type::Expression_1);
                    self.asm_data.put();
                    self.check_I();
                fi
            elif true
                self.error();
            fi

        elif self.check(sym::G64)
            if self.lb_seen
                self.lb_seen = false;
            fi

            self.asm_data.operand.enqueue(self.cur_sym);
            self.next_word();
            if self.check(sym::Arrow)
                self.next_word();
                if self.check(sym::M)
                    self.next_word();
                    if !(self.check(sym::G32) || self.check(sym::G64) || self.check(sym::FP))
                        self.error();
                    fi
                    self.asm_data.operand.enqueue(self.cur_sym);
                    self.next_word();
                    self.validate(sym::d8);
                    self.asm_data.operand.enqueue(self.cur_sym);
                    self.next_word();
                    self.validate(sym::Newline);
                    self.asm_data.set_i_type(i_type::Load);
                    self.asm_data.put();
                    self.check_I();
                elif self.check(sym::G64)
                    self.asm_data.operand.enqueue(self.cur_sym);
                    self.next_word();
                    self.validate(sym::BO);
                    self.asm_data.operand.enqueue(self.cur_sym);
                    self.next_word();
                    self.validate(sym::Arrow);
                    self.next_word();
                    self.validate(sym::V64);
                    self.asm_data.operand.enqueue(self.cur_sym);
                    self.next_word();
                    self.validate(sym::Newline);
                    self.asm_data.set_i_type(i_type::Expression_2);
                    self.asm_data.put();
                    self.check_I();
                elif true
                    self.error();
                fi
                
            elif self.check(sym::BO)
                self.asm_data.operand.enqueue(self.cur_sym);
                self.next_word();
                self.validate(sym::Arrow);
                self.next_word();
                if self.check(sym::Constant) || self.check(sym::G32)
                    self.asm_data.operand.enqueue(self.cur_sym);
                    self.next_word();
                    self.validate(sym::Newline);
                    self.asm_data.set_i_type(i_type::Expression_1);
                    self.asm_data.put();
                    self.check_I();
                elif true
                    self.error();
                fi
            fi

        elif self.check(sym::lb)
            if self.lb_seen
                self.error();
            elif !self.lb_seen
                self.lb_seen = true;
                self.sbr_data.define_lb(self.cur_sym);
                self.asm_data.set_i_type(i_type::Label);
                self.asm_data.operand.enqueue(self.cur_sym);
                self.asm_data.put();
                self.check_I();
            fi

        elif self.check(sym::CI)
            self.next_word();
            if self.check(sym::G32)
                self.asm_data.operand.enqueue(self.cur_sym);
                self.next_word();
                self.validate(sym::CO);
                self.asm_data.operand.enqueue(self.cur_sym);
                self.next_word();
                self.validate(sym::G32);
                self.asm_data.operand.enqueue(self.cur_sym);
                self.next_word();
                self.validate(sym::lb);
                self.asm_data.operand.enqueue(self.cur_sym);
                self.sbr_data.use_lb(self.cur_sym);
                self.asm_data.set_i_type(i_type::Compare);
                self.asm_data.put();
                self.check_I();
            elif self.check(sym::G32)
                self.asm_data.operand.enqueue(self.cur_sym);
                self.next_word();
                self.validate(sym::CO);
                self.asm_data.operand.enqueue(self.cur_sym);
                self.next_word();
                self.validate(sym::G64);
                self.asm_data.operand.enqueue(self.cur_sym);
                self.next_word();
                self.validate(sym::lb);
                self.asm_data.operand.enqueue(self.cur_sym);
                self.sbr_data.use_lb(self.cur_sym);
                self.asm_data.set_i_type(i_type::Compare);
                self.asm_data.put();
                self.check_I();
            elif !(self.check(sym::G32) || self.check(sym::G64))
                self.error();
            fi
        fi
    rbs

    sbr check_all_definitions(&self) -> bool
        decl var output = false;
        if self.sbr_data.all_sbr_defined()
            if self.sbr_data.all_lb_defined()
                output = true;
            fi
        fi
	return output
    rbs

    sbr next_word(&self)
        self.cur_sym = sym::Nothing;
        self.o_type = operation_type::Nothing;
        self.i = self.j;
        self.skip_whitespaces();
        self.j = self.i;
        decl var k : usize;

        if is_EOF(self.s[self.j])
            self.cur_sym = sym::EOF;
            
        elif is_OParen(self.s[self.j])
            self.cur_sym = sym::OParen;
        
        elif is_CParen(self.s[self.j])
            self.cur_sym = sym::CParen;
    
        elif is_Dollar(self.s[self.j])
            self.j += 1;
            k = self.j;
            if is_Alpha(self.s[self.j])
                while is_alnum(self.s[self.j]) do
                    self.j += 1;
                od
                if is_white_space(self.s[self.j])
                    self.cur_sym = sym::S(new word(k, self.j - 1));
                fi
            fi
        
        elif is_Colon(self.s[self.j])
            self.j += 1;
            k = self.j;
            if is_Alpha(self.s[self.j])
                while is_alnum(self.s[self.j]) do
                    self.j += 1;
                od
                if is_white_space(self.s[self.j])
                    self.cur_sym = sym::lb(new word(k, self.j - 1));
                fi
            fi
    
        elif is_Hash(self.s[self.j])
            self.j += 1;
            k = self.j;
            if is_Zero(self.s[self.j])
                self.j += 1;
                if is_X(self.s[self.j])
                    self.j += 1;
                    if is_Hex(self.s[self.j])
                        while is_Hex(self.s[self.j]) do
                            self.j += 1;
                        od
                        if is_white_space(self.s[self.j])
                            self.cur_sym = sym::d8(new word(k, self.j - 1));
                        fi
                    fi
                elif is_Num(self.s[self.j])
                    self.j += 1;
                    while is_Num(self.s[self.j]) do
                        self.j += 1;
                    od
                    if is_white_space(self.s[self.j])
                        self.cur_sym = sym::Num(new word(k, self.j - 1));
                    fi
                fi
            elif is_Num(self.s[self.j])
                self.j += 1;
                while is_Num(self.s[self.j]) do
                    self.j += 1;
                od
                if is_white_space(self.s[self.j])
                    self.cur_sym = sym::Num(new word(k, self.j - 1));
                fi
            fi

        elif is_Mem(self.s[self.j])
            self.j += 1;
            if is_E(self.s[self.j])
                self.j += 1;
                if is_M(self.s[self.j])
                    self.j += 1;
                    if is_white_space(self.s[self.j])
                        self.cur_sym = sym::M;
                    fi
                fi
            fi
         
        elif is_G32(self.s[self.j])
            self.j += 1;
            k = self.j;
            if is_Num(self.s[self.j])
                self.j += 1;
                if is_Num(self.s[self.j])
                    self.j += 1;
                    if is_white_space(self.s[self.j])
                        self.cur_sym = sym::G32(self.string_to_num(k, self.j - 1));
                    fi
                elif is_white_space(self.s[self.j])
                    self.cur_sym = sym::G32(self.string_to_num(k, self.j - 1));
                fi
            fi
    
        elif is_G64(self.s[self.j])
            self.j += 1;
            k = self.j;
            if is_Num(self.s[self.j])
                self.j += 1;
                if is_Num(self.s[self.j])
                    self.j += 1;
                    if is_white_space(self.s[self.j])
                        self.cur_sym = sym::G64(self.string_to_num(k, self.j - 1));
                    fi
                elif is_white_space(self.s[self.j])
                    self.cur_sym = sym::G64(self.string_to_num(k, self.j - 1));
                fi
            elif is_S(self.s[self.j])
                self.j += 1;
                if is_P(self.s[self.j])
                    self.j += 1;
                    if is_white_space(self.s[self.j])
                        self.cur_sym = sym::G64;
                    fi
                fi
            fi

	elif is_F(self.s[self.j])                                       
            self.j += 1;				
            if is_P(self.s[self.j])
                self.j += 1;
                if is_white_space(self.s[self.j])
                    self.cur_sym = sym::FP;
                fi
            fi
        
        elif is_Num(self.s[self.j])
            self.j += 1;
            while is_Num(self.s[self.j]) do
                self.j += 1;
            od
            if is_white_space(self.s[self.j])
                self.cur_sym = sym::Num(new word(self.i, self.j - 1));
            fi

        -- check for arrow and left shift operator both
        elif is_LShift(self.s[self.j])                     
            self.j += 1;
            if is_LShift(self.s[self.j])
                self.j += 1;
                if is_white_space(self.s[self.j])
                    self.cur_sym = sym::BO;
                    self.asm_data.set_o_type(operation_type::LSL);
                fi
            elif is_hyphen(self.s[self.j])
                self.j += 1;
                if is_white_space(self.s[self.j])
                    self.cur_sym = sym::Arrow;
                fi
            elif is_Equal(self.s[self.j])
                self.j += 1;
                if is_white_space(self.s[self.j])
                    self.cur_sym = sym::CO;
                    self.asm_data.set_o_type(operation_type::LESSEQUAL);
                fi
            elif is_white_space(self.s[self.j])
                self.j += 1;
                self.cur_sym = sym::CO;
                self.asm_data.set_o_type(operation_type::LESS);
            fi

        elif is_RShift(self.s[self.j])
            self.j += 1;
            if is_RShift(self.s[self.j])
                self.j += 1;
                if is_white_space(self.s[self.j])
                    self.cur_sym = sym::BO;
                    self.asm_data.set_o_type(operation_type::RSL);
                fi
            elif is_Equal(self.s[self.j])
                self.j += 1;
                if is_white_space(self.s[self.j])
                    self.cur_sym = sym::CO;
                    self.asm_data.set_o_type(operation_type::GREATEREQUAL);
                fi
            elif is_white_space(self.s[self.j])
                self.j += 1;
                self.cur_sym = sym::CO;
                self.asm_data.set_o_type(operation_type::GREATER);
            fi

        -- check for * - + operator
        elif is_BO(self.s[self.j])                            
                self.j += 1;
                if is_white_space(self.s[self.j])
                    self.cur_sym = sym::BO;
                    self.set_o_type();
                fi
        elif is_C(self.s[self.j])
            self.j += 1;
            if is_B(self.s[self.j])
                self.j += 1;
                if is_white_space(self.s[self.j])
                        self.cur_sym = sym::CI;
                fi
            fi
        fi
    rbs

    sbr set_o_type(&var self)
        if is_Plus(self.s[self.i])
            self.asm_data.set_o_type(operation_type::Add);
        elif is_Minus(self.s[self.i])
            self.asm_data.set_o_type(operation_type::Sub);
        elif is_Mult(self.s[self.i])
            self.asm_data.set_o_type(operation_type::Mul);
        fi
    rbs

    sbr skip_whitespaces(&self)
        while is_white_space(&self.s[self.i]) do
            self.i += 1;
        od
    rbs

    sbr check(&self, t: sym) -> bool
        decl var output = (self.cur_sym == t);
	return output
    rbs

    sbr validate(&self, t: sym)
        if self.cur_sym != t
            self.error();
        fi
    rbs

    sbr error(&self)
        -- exit unconditionally
    rbs
lpmi

impl AsmData
    sbr initialize(&var self, s : DArray<char>, d : DArray<char>)
        self.s = s;
        self.d = d;
        self.i = 0;
        self.operand = Queue::new();
        self.o_type = operation_type::Nothing;
    rbs
    
    sbr set_i_type(&var self, t : i_type)
        self.cur_i_type = t;
    rbs

    sbr put(&var self)
        if self.cur_i_type == i_type::Branch_LB
            self.add(new String("B __"));
            self.copy_from_operand(self.sbr_name);
            self.add(new String("__"));
            self.copy_from_operand(self.operand.dequeue());
            self.add(new String(" \n"));
        elif self.cur_i_type == i_type::Branch_SBR
            self.add(new String("BL "));
            self.copy_from_operand(self.operand.dequeue());
            self.add(new String(" \n"));
        elif self.cur_i_type == i_type::Load
            self.add(new String("LDR "));
            self.copy_from_operand(self.operand.dequeue());
            self.add(new String(" , [ "));
            self.copy_from_operand(self.operand.dequeue());
            self.add(new String(" , #"));
            self.copy_from_operand(self.operand.dequeue());
            self.add(new String(" ] \n"));
        elif self.cur_i_type == i_type::Store
            decl var t1 : word = self.operand.dequeue();
            decl var t2 : word = self.operand.dequeue();
            self.add(new String("STR "));
            self.copy_from_operand(self.operand.dequeue());
            self.add(new String(" , [ "));
            self.copy_from_operand(t1);
            self.add(new String(" , #"));
            self.copy_from_operand(t2);
            self.add(new String(" ] \n"));
        elif self.cur_i_type == i_type::Return
            self.add(new String("B __"));
            self.copy_from_operand(self.sbr_name);
            self.add(new String("__exit__"));
        elif self.cur_i_type == i_type::Function_Start
            self.copy_from_operand(self.sbr_name);
            self.add(new String(": \n"));
            self.add(new String("Str FP, #[SP, #-8]! \n"));
            self.add(new String("Str LR, #[SP, #-8]! \n"));
            self.add(new String("Mov FP, SP \n"));
        elif self.cur_i_type == i_type::Function_End
            self.add(new String("__"));
            self.copy_from_operand(self.sbr_name);
            self.add(new String("__exit__: \n"));
            self.add(new String("Mov SP, FP \n"));
            self.add(new String("LDR LR, #[SP], #8 \n"));
            self.add(new String("LDR FP, #[SP], #8 \n"));
            self.add(new String("ret \n"));
        elif self.cur_i_type == i_type::Label
            self.add(new String("__"));
            self.copy_from_operand(self.sbr_name);
            self.add(new String("__"));
            self.copy_from_operand(self.operand.dequeue());
            self.add(new String(": "));
        elif self.cur_i_type == i_type::Expression_1
            if self.o_type == operation_type::Add
                self.add(new String("Add "));
            elif self.o_type == operation_type::Sub
                self.add(new String("Sub "));
            elif self.o_type == operation_type::Mul
                self.add(new String("Mul "));
            fi
            self.copy_from_operand(self.operand.dequeue());
            self.add(new String(", "));
            self.copy_from_operand(self.operand.dequeue());
            self.add(new String(", "));
            self.copy_from_operand(self.operand.dequeue());
            self.add(new String(" \n"));
        elif self.cur_i_type == i_type::Expression_2
            if self.o_type == operation_type::Add
                self.add(new String("Add "));
            elif self.o_type == operation_type::Sub
                self.add(new String("Sub "));
            elif self.o_type == operation_type::Mul
                self.add(new String("Mul "));
            fi
            decl var t : word = self.operand.dequeue();
            self.copy_from_operand(t);
            self.add(new String(", "));
            self.copy_from_operand(t);
            self.add(new String(", "));
            self.copy_from_operand(self.operand.dequeue());
            self.add(new String(" \n"));
        elif self.cur_i_type == i_type::Compare
            self.add(new String("CMP "));
            self.copy_from_operand(self.operand.dequeue());
            self.add(new String(", "));
            self.copy_from_operand(self.operand.dequeue());
            self.add(new String(" \n"));
            if self.o_type == operation_type::LESS
                self.add(new String("BLT "));
            elif self.o_type == operation_type::LESSEQUAL
                self.add(new String("BLE "));
            elif self.o_type == operation_type::GREATER
                self.add(new String("BGT "));
            elif self.o_type == operation_type::GREATEREQUAL
                self.add(new String("BGE "));
            elif self.o_type == operation_type::EQUAL
                self.add(new String("BE "));
            elif self.o_type == operation_type::NOTEQUAL
                self.add(new String("BNE "));
            fi
            self.copy_from_operand(self.operand.dequeue());
            self.add(new String(" \n"));
        fi
    rbs

    sbr set_o_type(&var self, t : operation_type)
        self.o_type = t;
    rbs

    sbr add(&var self, t : String)
        decl var k : usize = 0;
        while k < t.length() do
            self.d[self.i] = t[k];
            k += 1;
            self.i += 1;
        od
    rbs
    
    sbr copy_from_operand(&var self, t : word)
        decl var k : usize = t.a;
        while k <= t.b do
            self.d[self.i] = self.s[k];
            k += 1;
            self.i += 1;
        od
    rbs
    
    sbr set_sbr_name(&var self, t : word)
        self.sbr_name = t;
    rbs
lpmi

impl SbrInfo

    sbr define_sbr(&self , sbr_name : word) -> bool
        decl var output = true;
        decl var k : Maybe<usize> = self.search_sbr(sbr_name);
        if k == None
            self.insert(sbr_name, true);
        elif k != None
	    -- already defined then redefinition error
            if self.sbr_data[k.unwrap()].is_defined                         
                output = false;
            elif true
                self.sbr_data[k.unwrap()].is_defined = true;
            fi
        fi
	return output
    rbs

    sbr use_sbr(&self , sbr_name : word)                                    
	-- store if not already defined
        decl var k : Maybe<usize> = self.search_sbr(sbr_name);
        if k == None
            self.insert_sbr(sbr_name, false);
        fi
    rbs
    
    sbr all_sbr_defined(&self) -> bool
        decl var output = true;
        decl var k : usize = 0;

        while output && k < self.sbr_data.length() do
            if self.sbr_data[k].is_defined == false
                output = false;
            fi
            k += 1;
        od
	return output
    rbs

    sbr define_lb(&self , sbr_name : word, lb_name : word) -> bool                      
	    -- define lb_name in sbr_name
        decl var output = true;
        decl var k : Maybe<usize> = self.search_sbr(sbr_name);
        decl var j : Maybe<usize> = self.search_lb(k, lb_name);
        if j == None
            self.insert_lb(k, lb_name, true);                                                -- not present
        elif j != None
            if self.sbr_data[k].labels[j.unwrap()].is_defined                           
		        -- redefinition
                output = false;
            elif true
                self.sbr_data[k].labels[j.unwrap()].is_defined = true;
            fi
        fi
	return output
    rbs

    sbr use_lb(&self , sbr_name : word, lb_name : word) -> bool                         
	-- store if not already defined
        decl var output = true;
        decl var k : Maybe<usize> = self.search_sbr(sbr_name);
        decl var j : Maybe<usize> = self.search_lb(k, lb_name);
        if j == None
            self.insert_lb(k, lb_name, false);
        fi
	return output
    rbs

    sbr all_lb_defined(&self) -> bool
        decl var output = true;
        decl var k1 : usize = 0;

        while output && k1 < self.sbr_data.length() do
            decl var k2 : usize = 0;
            while output && k2 < self.sbr_data[k1].labels.length() do
                if self.sbr_data[k1].labels[k2].is_defined == false
                    output = false;
                fi
                k2 += 1;
            od
            k1 += 1;
        od
	return output
    rbs

    sbr search_sbr(&self, sbr_name : word) -> Maybe<usize>
        decl var output = None;
        decl var k : usize = 0;
        decl var low : usize = 0;
        decl var high : usize = self.sbr_data.length();

        while k != self.sbr_data.length() && output == None do
            if self.compare(sbr_name, self.sbr_data[k],sbr_name) == cmp::EQUAL
                output = Some(k);
            fi
        od
	return output
    rbs

    sbr search_lb(&self, sbr_index : usize, lb_name : word) -> Maybe<usize>
        decl var output = None;
        decl var k : usize = 0;
        decl var low : usize = 0;
        decl var high : usize = self.sbr_data[sbr_index].labels.length();

        while (k != self.sbr_data[sbr_index].labels.length()) && output == None do
            if self.compare(lb_name, self.sbr_data[sbr_index].labels[k].lb_name, lb_name) == cmp::EQUAL
                output = Some(k);
            fi
        od
	return output
    rbs

    sbr insert_sbr(&self, sbr_name : word, defined : bool) 
        -- insert at sorted order
        decl var sd : SbrData = self.get_new_SbrData(sbr_name, defined);
        self.sbr_data.enqueue(sd);
        decl var k : usize = self.sbr_data.length() - 1;

        while k >= 0 && (self.compare(sbr_name, self.sbr_data[k]) == cmp::LESS) do
            self.sbr_data[k + 1] = self.sbr_data[k];
            k -= 1;
        od

        self.sbr_data[k + 1] = sd;
    rbs
	
	sbr get_new_SbrData(&self, sbr_name_ : word, defined : bool) -> SbrData
		decl var output = SbrData{		
    		sbr_name : sbr_name_,
    		sbr_defined : defined, 
    		labels : DArray::new(),
			};
		return output
	rbs

    sbr insert_lb(&self, sbr_index : usize, lb_name : word, defined : bool)        
        decl var ld : LabelData = self.get_new_LabelData(lb_name, defined);
        self.sbr_data[sbr_index].labels.enqueue(ld);
        decl var k : usize = self.sbr_data.length() - 1;

        while k >= 0 && (self.compare(lb_name, self.sbr_data[sbr_index].labels[k]) == cmp::LESS) do
            self.sbr_data[sbr_index].labels[k + 1] = self.sbr_data[sbr_index].labels[k];
            k -= 1;
        od

        self.sbr_data[sbr_index].labels[k + 1] = ld;
    rbs

    sbr compare(&self, word_1 : word, word_2 : word) -> cmp
        -- compare and return (LESS -> w1 < w2) (EQUAL -> w1 == w2) (GRETAER -> w2 > w1)
    rbs

lpmi

impl Queue
    sbr enqueue(&var self, t : word)
    rbs

    sbr dequeue(&var self) -> word
    rbs
lpmi



sbr is_white_space(t : char) -> bool
    decl output = ((t == ' ') || (t == '\t'));
	return output
rbs

sbr is_EOF(t : char) -> bool
	decl output = (t == '\0'); 
	return output
rbs

sbr is_hyphen(t : char) -> bool
	decl output = (t == '-'); 
	return output
rbs

sbr is_OParen(t : char) -> bool
    decl output = (t == '(');
	return output
rbs

sbr is_CParen(t : char) -> bool
	decl output = (t == ')');
	return output
rbs

sbr is_Hex(t : char) -> bool
	decl output = is_Num(t) || (t >= 'A' && t <= 'F');
	return output
rbs

sbr is_Dollar(t : char) -> bool
	decl output = (t == '$');
	return output
rbs

sbr is_Colon(t : char) -> bool
    decl output = (t == ':');
	return output
rbs

sbr is_Hash(t : char) -> bool
    decl output = (t == '#');
	return output
rbs

sbr is_Mem(t : char) -> bool
    decl output = ((t == 'm') || (t == 'M'));
	return output
rbs

sbr is_G32(t : char) -> bool
    decl output = (t == 'W');
	return output
rbs

sbr is_G64(t : char) -> bool
    decl output = (t == 'X');
	return output
rbs

sbr is_Num(t : char) -> bool
    decl output = (t >= '0' && t <= '9');
	return output
rbs

sbr is_LShift(t : char) -> bool
    decl output = (t == '<');
	return output
rbs

sbr is_RShift(t : char) -> bool
    decl output = (t == '>');
	return output
rbs

sbr is_BO(t : char) -> bool
    decl output = ((t == '+') || (t == '*') || (t == '-'));
	return output
rbs

sbr is_F(t : char) -> bool
    decl output = (t == 'F');
	return output
rbs

sbr is_P(t : char) -> bool
    decl output = (t == 'P');
	return output
rbs

sbr is_C(t : char) -> bool
    decl output = (t == 'C');
	return output
rbs

sbr is_B(t : char) -> bool
    decl output = (t == 'B');
	return output
rbs

sbr is_E(t : char) -> bool
    decl output = (t == 'e' || t == 'E');
	return output
rbs

sbr is_M(t : char) -> bool
    decl output = (t == 'M');
	return output
rbs

sbr is_X(t : char) -> bool
    decl output = (t == 'X');
	return output
rbs

sbr is_S(t : char) -> bool
    decl output = (t == 'S');
	return output
rbs

sbr is_Zero(t : char) -> bool
    decl output = (t == '0');
	return output
rbs

sbr is_alnum(t : char) -> bool
    decl output = (is_Alpha(t) || is_Num(t));
	return output
rbs

sbr is_Alpha(t : char) -> bool
    decl output = (t <= 'A' && t <= 'Z') || (t <= 'a' && t <= 'z');
	return output
rbs

sbr is_Plus(t : char) -> bool
    decl output = (t == '+');
	return output
rbs

sbr is_Minus(t : char) -> bool
    decl output = (t == '-');
	return output
rbs

sbr is_Mult(t : char) -> bool
    decl output = (t == '*');
	return output
rbs

sbr is_Equal(t : char) -> bool
    decl output = (t == '=');
	return output
rbs
