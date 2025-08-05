aggr DU_Data
    line_no     : u64,
    defined_var : Set,
    used_var    : Set,
    succ        : Set,
    in          : Set,
    out         : Set
rgga

enum Side
    LHS, RHS
mune

decl var cur_line : u64;
decl var cur_side : Side;
decl var graph = Graph::new();
decl var du_data : DArray<DU_Data> = DArray::new();

sbr variable(s : DArray<char>, d : DArray<char>)
    def_and_use();
    in_and_out();
    color_graph();
rbs

sbr color_graph()
    sort_by_degree();

    decl var colored_vertices = 0;
    decl var next_color = 1;
    decl var i : usize = 0;                                         // start coloring with highest degree vertex
    decl var available_colors : Set<u64> = Set::new();
    decl var neighbors : Set = Set::new();

    available_colors.addElement(next_color);

    while colored_vertices != graph.length() do
        if graph.vertices[i].color == 0
            fill_neighbors(i, neighbors);
            available_colors = difference(available_colors, neighbors);
            if available_colors.noOfElements == 0
                next_color += 1;
                available_colors.addElement(next_color);
                graph.vertices[i].color = next_color;
            elif available_colors.noOfElements != 0
                graph.vertices[i].color = available_colors.start.next();
            fi
            colored_vertices += 1;
        fi
    od
rbs

sbr fill_neighbors(i : usize, neighbors : Set)
    neighbors.removeAll();
    decl var j = graph.neighbors[i].start();
    
    while j != None do
        neighbors.addElement(j.next().color);
    od

    neighbors = difference(neighbors, Set::new({0}));
rbs

sbr in_and_out()
    decl var k : u64 = 0;
    decl var prev_count : u64 = 0;                  // do until no more changes
    decl var count : u64 = 0;

    do
        prev_count = count;
        count = 0;

        while k != du_data.length() do
            du_data[k].in = union(du_data[k].used_var, difference(du_data[k].out, du_data[k].define_var));

            // for all successors
            decl var j = du_data[k].succ.start();
            while j != None do
                du_data[k].out = union(du_data[j.next().unwrap()].in, du_data[k].out);
            od

            count += du_data[k].in.noOfElements();
            count += du_data[k].out.noOfElements();

            graph.make_edges_from_set(du_data[k].in);
            graph.make_edges_from_set(du_data[k].out);
        od
        
    od while prev_count < count
    
rbs

sbr def_and_use()
    cur_line = 0;
    du_data.push(get_new_DU_Data(cur_line));

    while cur_sym != CParenthesis do
        next_word();
        if check(Variable)
            if cur_side == LHS
                define_var();
            elif cur_side == RHS
                use_var();
            fi

        elif check(Arrow)
            cur_side = Side::RHS;

        elif check(NewLine)
            cur_line += 1;
            cur_side = Side::LHS;
            du_data.push(get_new_DU_Data(cur_line));
            du_data[cur_line].succ.addElement(cur_line + 1);

        elif check(Branch)
            next_word();
            du_data[cur_line].succ.removeElement(cur_line + 1);
            decl var k = find_label_line_no(cur_word);
            du_data[cur_line].succ.addElement(k);

        elif check(Compare_Branch)
            next_word();
            if check(Variable)
                use_var();
            fi
            next_word();
            next_word();
            if check(Variable)
                use_var();
            fi
            next_word();
            decl var k = find_label_line_no(cur_word);
            du_data[cur_line].succ.addElement(k);
        fi
    od

    du_data[cur_line].succ.removeElement(cur_line + 1);
rbs

sbr define_var()
    du_data[cur_line].defined_var.addElement(cur_word);
    graph.add_vertex(cur_word);
rbs

sbr use_var()
    du_data[cur_line].used_var.addElement(cur_word);
rbs

sbr find_label_line_no(label : String) -> u64
    // find the line_no on which label was defined
rbs

sbr get_new_DU_Data(line_no : u64) -> DU_Data
    // return new instance of DU_Data
rbs

aggr Set

rgga

impl Set
    sbr noOfElements() -> u64
    rbs
    
    sbr addElement()
    rbs

    sbr removeElement()
    rbs

    sbr removeAll()
    rbs

    // type of iterator
    sbr start()
    rbs

    sbr next() -> Maybe<T>
    rbs
lpmi

sbr union(s1 : Set, s2 : Set) -> Set
rbs

sbr difference(s1 : Set, s2 : Set) -> Set
rbs


aggr List

rgga

impl List
    sbr contains(t : Some_Type) -> bool
    rbs

    sbr add(t : Some_Type)
    rbs
lpmi

aggr Vertex
    data : String       // for now
    color : u64         // 0 -> non-colored else colored with number i
rgga

aggr Graph
    // if vertices[i] is vertex x the neighbors[i] contains neighbors of x
    vertices : DArray<Vertex>,                        
    neighbors : DArray<List<Vertex>>,
rgga

impl Graph
    sbr find_vertex(v : String) -> Maybe<usize>
        return vertices.contains(v);
    rbs

    sbr add_vertex(v : String)
        // if not already exists add the vertex
        if find_vertex(v) == None
            vertices.add(v);
        fi
    rbs

    sbr add_edge(v1 : String, v2 : String)
        // if not already exists add the edge
        decl var k1 = find_vertex(v1);
        decl var k2 = find_vertex(v2);

        if neighbors[k1.unwrap()].contains(v2) == false
            neighbors[k1.unwrap()].add(v2);
            neighbors[k2.unwrap()].add(v1);
        fi
    rbs

    sbr make_edges_from_set(s : Set)
        // for all possible pair in set add an edge
        decl var j = s.start();
        while j != None do
            decl var vx = j.next();
            decl var k = j;
            k.next();
            while k != None do
                add_edge(vx, k.next());
            od
        od
    rbs
lpmi
