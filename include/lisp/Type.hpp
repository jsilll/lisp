#pragma once

namespace lisp
{
    /// @brief The type of a value.
    enum class Type
    {
        Unit,
        Atom,
        Int,
        Float,
        List,
        String,
        Quote,
        Lambda,
        Builtin,
    };
} // namespace lisp
