#include <lisp/Token.hpp>

namespace lisp {
std::string
Token::ToString() const noexcept {
    switch (type) {
    case Type::LeftParen:
        return "Token{Type::LeftParen, " + position.ToString() + "}";
    case Type::RightParen:
        return "Token{Type::RightParen, " + position.ToString() + "}";
    case Type::Quote:
        return "Token{Type::Quote, " + position.ToString() + "}";
    case Type::Symbol:
        return "Token{Type::Symbol, " + position.ToString() + ", '" +
               std::string(lex) + "'}";
    case Type::Integer:
        return "Token{Type::Integer, " + position.ToString() + ", '" +
               std::string(lex) + "'}";
    case Type::Float:
        return "Token{Type::Float, " + position.ToString() + ", '" +
               std::string(lex) + "'}";
    case Type::String:
        return "Token{Type::String, " + position.ToString() + ", '" +
               std::string(lex) + "'}";
    case Type::Eof:
        return "Token{Type::Eof, " + position.ToString() + "}";
    case Type::Invalid:
        return "Token{Type::Invalid, " + position.ToString() + "}";
    }
}
}   // namespace lisp
