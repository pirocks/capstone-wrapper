pub mod tokenize;
pub mod parser;

#[cfg(test)]
pub mod test {
    use crate::parser::Tokens;
    use crate::tokenize::{Lexer, remove_whitespace};

    #[test]
    pub fn test() {
        let ses = [r#"
        updateMap(RSMap,
convToRegKeys(R2) |-> extractMInt( addMInt( (#ifMInt eqMInt(getFlag("CF", RSMap), mi(1,1)) #then addMInt( concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)), mi(65, 1)) #else concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)) #fi), concatenateMInt( mi(1, 0), getParentValue(R2, RSMap))), 1, 65)

"CF" |-> extractMInt( addMInt( (#ifMInt eqMInt(getFlag("CF", RSMap), mi(1,1)) #then addMInt( concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)), mi(65, 1)) #else concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)) #fi), concatenateMInt( mi(1, 0), getParentValue(R2, RSMap))), 0, 1)

"PF" |-> (#ifMInt (notBool (((((((eqMInt( extractMInt( addMInt( (#ifMInt eqMInt(getFlag("CF", RSMap), mi(1,1)) #then addMInt( concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)), mi(65, 1)) #else concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)) #fi), concatenateMInt( mi(1, 0), getParentValue(R2, RSMap))), 64, 65), mi(1, 1)) xorBool eqMInt( extractMInt( addMInt( (#ifMInt eqMInt(getFlag("CF", RSMap), mi(1,1)) #then addMInt( concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)), mi(65, 1)) #else concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)) #fi), concatenateMInt( mi(1, 0), getParentValue(R2, RSMap))), 63, 64), mi(1, 1))) xorBool eqMInt( extractMInt( addMInt( (#ifMInt eqMInt(getFlag("CF", RSMap), mi(1,1)) #then addMInt( concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)), mi(65, 1)) #else concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)) #fi), concatenateMInt( mi(1, 0), getParentValue(R2, RSMap))), 62, 63), mi(1, 1))) xorBool eqMInt( extractMInt( addMInt( (#ifMInt eqMInt(getFlag("CF", RSMap), mi(1,1)) #then addMInt( concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)), mi(65, 1)) #else concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)) #fi), concatenateMInt( mi(1, 0), getParentValue(R2, RSMap))), 61, 62), mi(1, 1))) xorBool eqMInt( extractMInt( addMInt( (#ifMInt eqMInt(getFlag("CF", RSMap), mi(1,1)) #then addMInt( concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)), mi(65, 1)) #else concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)) #fi), concatenateMInt( mi(1, 0), getParentValue(R2, RSMap))), 60, 61), mi(1, 1))) xorBool eqMInt( extractMInt( addMInt( (#ifMInt eqMInt(getFlag("CF", RSMap), mi(1,1)) #then addMInt( concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)), mi(65, 1)) #else concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)) #fi), concatenateMInt( mi(1, 0), getParentValue(R2, RSMap))), 59, 60), mi(1, 1))) xorBool eqMInt( extractMInt( addMInt( (#ifMInt eqMInt(getFlag("CF", RSMap), mi(1,1)) #then addMInt( concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)), mi(65, 1)) #else concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)) #fi), concatenateMInt( mi(1, 0), getParentValue(R2, RSMap))), 58, 59), mi(1, 1))) xorBool eqMInt( extractMInt( addMInt( (#ifMInt eqMInt(getFlag("CF", RSMap), mi(1,1)) #then addMInt( concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)), mi(65, 1)) #else concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)) #fi), concatenateMInt( mi(1, 0), getParentValue(R2, RSMap))), 57, 58), mi(1, 1)))) #then mi(1, 1) #else mi(1, 0) #fi)

"AF" |-> xorMInt( xorMInt( extractMInt( getParentValue(R1, RSMap), 59, 60), extractMInt( getParentValue(R2, RSMap), 59, 60)), extractMInt( addMInt( (#ifMInt eqMInt(getFlag("CF", RSMap), mi(1,1)) #then addMInt( concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)), mi(65, 1)) #else concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)) #fi), concatenateMInt( mi(1, 0), getParentValue(R2, RSMap))), 60, 61))

"ZF" |-> (#ifMInt eqMInt( extractMInt( addMInt( (#ifMInt eqMInt(getFlag("CF", RSMap), mi(1,1)) #then addMInt( concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)), mi(65, 1)) #else concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)) #fi), concatenateMInt( mi(1, 0), getParentValue(R2, RSMap))), 1, 65), mi(64, 0)) #then mi(1, 1) #else mi(1, 0) #fi)

"SF" |-> extractMInt( addMInt( (#ifMInt eqMInt(getFlag("CF", RSMap), mi(1,1)) #then addMInt( concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)), mi(65, 1)) #else concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)) #fi), concatenateMInt( mi(1, 0), getParentValue(R2, RSMap))), 1, 2)

"OF" |-> (#ifMInt ((eqMInt( extractMInt( getParentValue(R1, RSMap), 0, 1), mi(1, 1)) ==Bool eqMInt( extractMInt( getParentValue(R2, RSMap), 0, 1), mi(1, 1))) andBool (notBool (eqMInt( extractMInt( getParentValue(R1, RSMap), 0, 1), mi(1, 1)) ==Bool eqMInt( extractMInt( addMInt( (#ifMInt eqMInt(getFlag("CF", RSMap), mi(1,1)) #then addMInt( concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)), mi(65, 1)) #else concatenateMInt( mi(1, 0), getParentValue(R1, RSMap)) #fi), concatenateMInt( mi(1, 0), getParentValue(R2, RSMap))), 1, 2), mi(1, 1))))) #then mi(1, 1) #else mi(1, 0) #fi)
)
        "#,
            r#"updateMap(RSMap, ("RIP" |-> addMInt({RSMap["RIP"]}:>MInt, handleImmediateWithSignExtend(Imm32, 32, 64))))"#,
        r#"updateMap(RSMap,
convToRegKeys(R2) |-> xorMInt( getParentValue(R2, RSMap), getParentValue(R1, RSMap))

"CF" |-> mi(1, 0)

"PF" |-> (#ifMInt (notBool (((((((eqMInt( xorMInt( extractMInt( getParentValue(R2, RSMap), 63, 64), extractMInt( getParentValue(R1, RSMap), 63, 64)), mi(1, 1)) xorBool eqMInt( xorMInt( extractMInt( getParentValue(R2, RSMap), 62, 63), extractMInt( getParentValue(R1, RSMap), 62, 63)), mi(1, 1))) xorBool eqMInt( xorMInt( extractMInt( getParentValue(R2, RSMap), 61, 62), extractMInt( getParentValue(R1, RSMap), 61, 62)), mi(1, 1))) xorBool eqMInt( xorMInt( extractMInt( getParentValue(R2, RSMap), 60, 61), extractMInt( getParentValue(R1, RSMap), 60, 61)), mi(1, 1))) xorBool eqMInt( xorMInt( extractMInt( getParentValue(R2, RSMap), 59, 60), extractMInt( getParentValue(R1, RSMap), 59, 60)), mi(1, 1))) xorBool eqMInt( xorMInt( extractMInt( getParentValue(R2, RSMap), 58, 59), extractMInt( getParentValue(R1, RSMap), 58, 59)), mi(1, 1))) xorBool eqMInt( xorMInt( extractMInt( getParentValue(R2, RSMap), 57, 58), extractMInt( getParentValue(R1, RSMap), 57, 58)), mi(1, 1))) xorBool eqMInt( xorMInt( extractMInt( getParentValue(R2, RSMap), 56, 57), extractMInt( getParentValue(R1, RSMap), 56, 57)), mi(1, 1)))) #then mi(1, 1) #else mi(1, 0) #fi)

"AF" |-> (undefMInt)

"ZF" |-> (#ifMInt eqMInt( xorMInt( getParentValue(R2, RSMap), getParentValue(R1, RSMap)), mi(64, 0)) #then mi(1, 1) #else mi(1, 0) #fi)

"SF" |-> xorMInt( extractMInt( getParentValue(R2, RSMap), 0, 1), extractMInt( getParentValue(R1, RSMap), 0, 1))

"OF" |-> mi(1, 0)
)"#,
        r#"updateMap(RSMap,
convToRegKeys(R3) |-> concatenateMInt( add_double(extractMInt( getParentValue(R2, RSMap), 0, 64), extractMInt( getParentValue(R1, RSMap), 0, 64)), concatenateMInt( add_double(extractMInt( getParentValue(R2, RSMap), 64, 128), extractMInt( getParentValue(R1, RSMap), 64, 128)), concatenateMInt( add_double(extractMInt( getParentValue(R2, RSMap), 128, 192), extractMInt( getParentValue(R1, RSMap), 128, 192)), add_double(extractMInt( getParentValue(R2, RSMap), 192, 256), extractMInt( getParentValue(R1, RSMap), 192, 256)))))
)"#,r#"
updateMap(RSMap,
convToRegKeys(R3) |-> andMInt( extractMInt( lshrMInt( concatenateMInt( mi(448, 0), Mem64), uvalueMInt(concatenateMInt( mi(504, 0), extractMInt( getParentValue(R1, RSMap), 56, 64)))), 448, 512), negMInt( extractMInt( shiftLeftMInt( lshrMInt( mi(512, 18446744073709551615), uvalueMInt(concatenateMInt( mi(504, 0), extractMInt( getParentValue(R1, RSMap), 48, 56)))), uvalueMInt(concatenateMInt( mi(504, 0), extractMInt( getParentValue(R1, RSMap), 48, 56)))), 448, 512)))

"CF" |-> mi(1, 0)

"PF" |-> (undefMInt)

"AF" |-> (undefMInt)

"ZF" |-> (#ifMInt eqMInt( andMInt( extractMInt( lshrMInt( concatenateMInt( mi(448, 0), Mem64), uvalueMInt(concatenateMInt( mi(504, 0), extractMInt( getParentValue(R1, RSMap), 56, 64)))), 448, 512), negMInt( extractMInt( shiftLeftMInt( lshrMInt( mi(512, 18446744073709551615), uvalueMInt(concatenateMInt( mi(504, 0), extractMInt( getParentValue(R1, RSMap), 48, 56)))), uvalueMInt(concatenateMInt( mi(504, 0), extractMInt( getParentValue(R1, RSMap), 48, 56)))), 448, 512))), mi(64, 0)) #then mi(1, 1) #else mi(1, 0) #fi)

"SF" |-> (undefMInt)

"OF" |-> mi(1, 0)
      )
"#,
        r#"updateMap(RSMap,
convToRegKeys(R4) |-> concatenateMInt( mi(128, 0), extractMInt( lshrMInt( concatenateMInt( extractMInt( getParentValue(R3, RSMap), 128, 256), extractMInt( getParentValue(R2, RSMap), 128, 256)), uvalueMInt(shiftLeftMInt( concatenateMInt( mi(248, 0), handleImmediateWithSignExtend(Imm8, 8, 8)), uvalueMInt(mi(256, 3))))), 128, 256))
)"#];
        for s in ses {
            let tokens = remove_whitespace(Lexer::new(s.to_string()).tokens());
            dbg!(Tokens::new(tokens).parse_expression());
        }
    }
}

