use crate::lexer::tok::{Delimiters, TokenType};
use crate::parser::ast::{ArrayExpression, Expression, Literal};
use crate::parser::parser::Parser;
use crate::parser::parser_error::ParserError;
use crate::parser::parser_error::ParserErrorType::{UnexpectedToken, ValidationError};

impl Parser{

    /// Permet de parser un tableau pouvant contenir des "lignes" séparées par `;`.
    /// Exemple : [1.0, 2.0; 3.0, 4.0]
    /// Retourne un Expression::Array contenant un vecteur de lignes
    /// où chaque ligne est elle-même un Expression::Array.
    // pub fn parse_array_or_matrix_literal(&mut self) -> Result<Expression, ParserError> {
    //     // Consommer '['
    //     self.consume(TokenType::DELIMITER(Delimiters::LSBRACKET))?;
    //
    //     // On va stocker les lignes dans `rows`.
    //     // Chaque ligne sera un Vec<Expression>.
    //     let mut rows: Vec<Vec<Expression>> = Vec::new();
    //     let mut current_row: Vec<Expression> = Vec::new();
    //
    //     loop {
    //         // Vérifier si on a directement ']' : cas d'un tableau vide [ ]
    //         if self.check(&[TokenType::DELIMITER(Delimiters::RSBRACKET)]) {
    //             // on ferme, plus de données
    //             self.advance(); // Consomme le ']'
    //             break;
    //         }
    //
    //         // Sinon, on parse un élément.
    //         // Tu peux soit appeler parse_expression(0)? pour autoriser tout type d'expression
    //         // ou un parse d'un littéral si tu veux restreindre à int/float seulement.
    //         let element = self.parse_expression(0)?;
    //
    //         current_row.push(element);
    //
    //         // Maintenant on regarde le token suivant : ',', ';' ou ']'
    //         if let Some(token) = self.current_token() {
    //             match &token.token_type {
    //                 TokenType::DELIMITER(Delimiters::COMMA) => {
    //                     // On consomme la virgule et on continue la boucle
    //                     self.advance();
    //                     // on continue dans la même ligne
    //                 }
    //                 TokenType::DELIMITER(Delimiters::SEMICOLON) => {
    //                     // Fin de la "ligne" actuelle
    //                     self.advance(); // consomme ';'
    //                     rows.push(current_row);
    //                     current_row = Vec::new();
    //                 }
    //                 TokenType::DELIMITER(Delimiters::RSBRACKET) => {
    //                     // On ferme le tableau
    //                     self.advance();
    //                     // On push la dernière ligne si elle n'est pas vide
    //                     if !current_row.is_empty() {
    //                         rows.push(current_row);
    //                     }
    //                     break;
    //                 }
    //                 // Si c'est un autre token (ex. colon, operator, etc.), erreur
    //                 _ => {
    //                     return Err(ParserError::new(UnexpectedToken,
    //                         self.current_position(),
    //                     ));
    //                 }
    //             }
    //         } else {
    //             // On est à la fin de fichier (None)
    //             return Err(ParserError::new(UnexpectedEndOfInput, self.current_position()));
    //         }
    //     }
    //
    //     // Si on arrive ici, on a potentiellement un `rows` qui contient
    //     // plusieurs "lignes", chacune étant un Vec<Expression>.
    //
    //     // Pour faire un "tableau de tableaux", on convertit chaque ligne
    //     // en Expression::Array(...).
    //     let row_expressions: Vec<Expression> = rows
    //         .into_iter()
    //         .map(|row| Expression::Array(Box::new(ArrayExpression { elements: row })))
    //         .collect();
    //
    //     // Au final, on encapsule tout dans un Expression::Array(...)
    //     // contenant les lignes.
    //     // => [ [1.0, 2.0], [3.0, 4.0] ]
    //     let final_array = Expression::Array(Box::new(ArrayExpression {
    //         elements: row_expressions,
    //     }));
    //
    //     Ok(final_array)
    // }

    //
    // pub fn parse_array_or_matrix_literal(&mut self) -> Result<Expression, ParserError> {
    //     // Consommer '['
    //     self.consume(TokenType::DELIMITER(Delimiters::LSBRACKET))?;
    //
    //     let mut dimensions = Vec::new();
    //     let mut current_row = Vec::new();
    //     let mut all_elements = Vec::new();
    //
    //     loop {
    //         // Parse un élément (entier ou flottant)
    //         match &self.current_token().unwrap().token_type {
    //             TokenType::FLOAT { value } => {
    //                 current_row.push(Expression::Literal(Literal::Float { value: *value }));
    //                 self.advance();
    //             },
    //             TokenType::INTEGER { value } => {
    //                 current_row.push(Expression::Literal(Literal::Integer {
    //                     value: value.clone()
    //                 }));
    //                 self.advance();
    //             },
    //             _ => return Err(ParserError::new(UnexpectedToken, self.current_position())),
    //         }
    //
    //         // Vérifie le token suivant
    //         match &self.current_token().unwrap().token_type {
    //             TokenType::DELIMITER(Delimiters::COMMA) => {
    //                 self.advance();
    //                 continue;
    //             },
    //             TokenType::DELIMITER(Delimiters::SEMICOLON) => {
    //                 // Fin d'une ligne, ajouter la dimension si c'est la première ligne
    //                 if dimensions.is_empty() {
    //                     dimensions.push(current_row.len());
    //                 } else if current_row.len() != dimensions[dimensions.len() - 1] {
    //                     return Err(ParserError::new(ValidationError, self.current_position()));
    //                 }
    //
    //                 // Ajouter la ligne courante aux éléments
    //                 all_elements.extend(current_row.drain(..));
    //                 current_row = Vec::new();
    //                 self.advance();
    //             },
    //             TokenType::DELIMITER(Delimiters::RSBRACKET) => {
    //                 // Ajouter la dernière ligne
    //                 if !current_row.is_empty() {
    //                     if dimensions.is_empty() {
    //                         dimensions.push(current_row.len());
    //                     } else if current_row.len() != dimensions[dimensions.len() - 1] {
    //                         return Err(ParserError::new(ValidationError, self.current_position()));
    //                     }
    //                     all_elements.extend(current_row.drain(..));
    //                 }
    //                 self.advance();
    //                 break;
    //             },
    //             _ => return Err(ParserError::new(UnexpectedToken, self.current_position())),
    //         }
    //     }
    //
    //     // Ajouter le nombre de lignes comme dimension
    //     if !dimensions.is_empty() {
    //         dimensions.insert(0, (all_elements.len() / dimensions[0]));
    //     }
    //
    //     Ok(Expression::Array(Box::new(ArrayExpression {
    //         elements: all_elements,
    //         dimensions: Some(dimensions),
    //     })))
    // }

    pub fn parse_array_or_matrix_literal(&mut self) -> Result<Expression, ParserError> {
        match self.peek_token().unwrap().token_type {
            TokenType::DELIMITER(Delimiters::LSBRACKET) => {
                self.parse_nested_array(0)
            },
            _ => self.parse_flat_array()
        }
    }
    // Parse les tableaux à syntaxe plate: [1,2;3,4]
    fn parse_flat_array(&mut self) -> Result<Expression, ParserError> {
        self.consume(TokenType::DELIMITER(Delimiters::LSBRACKET))?;

        let mut dimensions = Vec::new();
        let mut current_row = Vec::new();
        let mut all_elements = Vec::new();

        loop {
            match &self.current_token().unwrap().token_type {
                TokenType::FLOAT { value } => {
                    current_row.push(Expression::Literal(Literal::Float { value: *value }));
                    self.advance();
                },
                TokenType::INTEGER { value } => {
                    current_row.push(Expression::Literal(Literal::Integer {
                        value: value.clone()
                    }));
                    self.advance();
                },
                _ => return Err(ParserError::new(UnexpectedToken, self.current_position())),
            }

            match &self.current_token().unwrap().token_type {
                TokenType::DELIMITER(Delimiters::COMMA) => {
                    self.advance();
                    continue;
                },
                TokenType::DELIMITER(Delimiters::SEMICOLON) => {
                    if dimensions.is_empty() {
                        dimensions.push(current_row.len());
                    } else if current_row.len() != dimensions[dimensions.len() - 1] {
                        return Err(ParserError::new(ValidationError, self.current_position()));
                    }
                    all_elements.extend(current_row.drain(..));
                    current_row = Vec::new();
                    self.advance();
                },
                TokenType::DELIMITER(Delimiters::RSBRACKET) => {
                    if !current_row.is_empty() {
                        if dimensions.is_empty() {
                            dimensions.push(current_row.len());
                        } else if current_row.len() != dimensions[dimensions.len() - 1] {
                            return Err(ParserError::new(ValidationError, self.current_position()));
                        }
                        all_elements.extend(current_row.drain(..));
                    }
                    self.advance();
                    break;
                },
                _ => return Err(ParserError::new(UnexpectedToken, self.current_position())),
            }
        }

        if !dimensions.is_empty() {
            dimensions.insert(0, (all_elements.len() / dimensions[0]));
        }

        Ok(Expression::Array(Box::new(ArrayExpression {
            elements: all_elements,
            dimensions: Some(dimensions),
        })))
    }

    // Parse les tableaux à syntaxe imbriquée: [[1,2],[3,4]]
    fn parse_nested_array(&mut self, depth: usize) -> Result<Expression, ParserError> {
        self.consume(TokenType::DELIMITER(Delimiters::LSBRACKET))?;

        let mut elements = Vec::new();
        let mut dimensions = Vec::new();
        let mut first_sub_dimensions: Option<Vec<usize>> = None;

        loop {
            match &self.current_token().unwrap().token_type {
                TokenType::DELIMITER(Delimiters::LSBRACKET) => {
                    let sub_array = self.parse_nested_array(depth + 1)?;
                    if let Expression::Array(ref array_expr) = sub_array {
                        // Vérifier la cohérence des dimensions des sous-tableaux
                        if let Some(sub_dims) = &array_expr.dimensions {
                            if let Some(first_dims) = &first_sub_dimensions {
                                if sub_dims != first_dims {
                                    return Err(ParserError::new(ValidationError, self.current_position()));
                                }
                            } else {
                                first_sub_dimensions = Some(sub_dims.clone());
                            }
                        }
                        elements.push(sub_array);
                    }
                },
                TokenType::FLOAT { value } => {
                    if depth == 0 {
                        return Err(ParserError::new(ValidationError, self.current_position()));
                    }
                    elements.push(Expression::Literal(Literal::Float { value: *value }));
                    self.advance();
                },
                TokenType::INTEGER { value } => {
                    if depth == 0 {
                        return Err(ParserError::new(ValidationError, self.current_position()));
                    }
                    elements.push(Expression::Literal(Literal::Integer {
                        value: value.clone()
                    }));
                    self.advance();
                },
                TokenType::DELIMITER(Delimiters::RSBRACKET) => {
                    self.advance();
                    break;
                },
                _ => return Err(ParserError::new(UnexpectedToken, self.current_position())),
            }

            match &self.current_token().unwrap().token_type {
                TokenType::DELIMITER(Delimiters::COMMA) => {
                    self.advance();
                    continue;
                },
                TokenType::DELIMITER(Delimiters::RSBRACKET) => {
                    continue;
                },
                _ => return Err(ParserError::new(UnexpectedToken, self.current_position())),
            }
        }

        // Calculer les dimensions
        dimensions.push(elements.len());
        if let Some(sub_dims) = first_sub_dimensions {
            dimensions.extend(sub_dims);
        }

        Ok(Expression::Array(Box::new(ArrayExpression {
            elements,
            dimensions: Some(dimensions),
        })))
    }







}