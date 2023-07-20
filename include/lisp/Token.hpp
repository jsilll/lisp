#pragma once

#include <lisp/Position.hpp>

namespace lisp
{
    /// @brief A token.
    struct Token
    {
        /// @brief The type of a token.
        enum class Type
        {
            LeftParen,
            RightParen,
            Quote,
            Symbol,
            Integer,
            Float,
            String,
            Eof,
            Invalid,
        };

        Type type;
        Position position;
        ::std::string_view lex;

        /// @brief Convert a token to a string.
        ::std::string ToString() const noexcept;
    };
} // namespace lisp
