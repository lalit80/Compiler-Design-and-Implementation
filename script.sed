s/--/\/\//g

s/\<aggr\>\(.*\)/struct\1 {/
s/rgga/}/

s/\<enum\>\(.*\)/enum\1 {/
s/mune/}/

s/\<impl\>\(.*\)/impl\1 {/
s/lpmi/}/

s/\<sbr\>\(.*\)/fn\1 {/
s/rbs/}/

s/\<do\>/{/
s/\<od\>/}/

s/length/len/g

s/\<decl\>/let/g
s/\<var\>/mut/g
s/DArray/Vec/g
s/\<Maybe\>/Option/g

s/\<if\>\(.*\)/\0 {/
s/\<elif\>\(.*\)/} else if\1 {/
s/\<fi\>\(.*\)/\1}/

s/new String("\(.*\)")/String::from("\1")/g
s/new word(\(.*\))/word::new(\1)/g

s/\r//g
