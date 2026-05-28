# KOSL Formal Grammar (EBNF)

```
document       = { pair | empty_line | comment } ;
pair           = identifier , "=" , value ;
```

```
value          = implicit_array 
               | object 
               | explicit_array 
               | scalar ;
```

```
implicit_array = scalar_or_struct , { "," , scalar_or_struct }+ ;
explicit_array = "[" , [ value , { "," , value } , [","] ] , "]" ;
object         = "(" , [ pair , { "," , pair } , [","] ] , ")" ;
```

```
scalar_or_struct= object | explicit_array | scalar ;
scalar         = string | bareword | number | boolean | "null" ;
```

```
identifier     = bareword ;
bareword       = ( ALPHA | DIGIT | "_" | "-" | "." )+ ;
string         = '"' , { ANY_CHAR - '"' } , '"' ;
comment        = ( "#" | "//" ) , { ANY_CHAR - NEWLINE } ;
```

# Type Inference Rules:
# 1. Barewords matching `true` or `false` are Booleans.
# 2. Barewords matching `null` are Null.
# 3. Barewords parsing as valid i64 are Integers.
# 4. Barewords parsing as valid f64 (single decimal) are Floats.
# 5. ALL OTHER BAREWORDS (e.g., `0.1.0`, `my_string`) are parsed as Strings.