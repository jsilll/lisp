#pragma once

#include "Lexer.hpp"
#include "Value.hpp"

namespace lisp
{
    /// @brief A parser for the Lisp language.
    class Parser final
    {
    public:
        /// @brief Constructs a parser from a source string.
        explicit Parser(Lexer lexer) noexcept : m_lexer(std::move(lexer)) {}

        /// @brief Parses the source string.
        Value Parse();

    private:
        /// @brief Parses a list.
        Value ParseList();

        /// @brief Parses an integer.
        Value ParseInteger(const Token &token);

        /// @brief Parses a float.
        Value ParseFloat(const Token &token);

        Lexer m_lexer;
    };
} // namespace lisp
