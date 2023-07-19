#pragma once

#include <vector>

#include <lisp/Type.hpp>
#include <lisp/Environment.hpp>

namespace lisp
{
    // Forward declaration.
    class Value;

    /// @brief A builtin function.
    typedef Value (*Builtin)(::std::vector<Value> args, const Environment &env);

    /// @brief A value in the lisp language.
    class Value final
    {
    public:
        Value() noexcept : m_type(Type::Unit) {}

        /// @brief Construct an integer value.
        explicit Value(int i) noexcept : m_type(Type::Int)
        {
            m_stack_data.i = i;
        }

        /// @brief Construct a floating point value.
        explicit Value(float f) noexcept : m_type(Type::Float)
        {
            m_stack_data.f = f;
        }

        /// @brief Construct a list value.
        explicit Value(::std::vector<Value> l) noexcept : m_type(Type::List),
                                                        m_list_data(::std::move(l)) {}

        /// @brief Construct a string or an atom value.
        Value(::std::string s, Type t);

        /// @brief Construct a builtin function.
        Value(::std::string s, Builtin b) noexcept : m_type(Type::Builtin), m_string_data(::std::move(s))
        {
            m_stack_data.b = b;
        }

        /// @brief Construct a lambda value.
        Value(::std::vector<Value> args, Value body, const Environment &scope) noexcept;

        /// @brief Construct a quote value.
        [[nodiscard]] Value Quote() const noexcept;

        /// @brief Get all the atoms in this value.
        [[nodiscard]] ::std::vector<::std::string> GetAtoms() const noexcept;

        /// @brief Get the string representation of this value.
        [[nodiscard]] ::std::string ToString() const noexcept;

        /// @brief Evaluate this value.
        Value Eval(const Environment &env) const;

        /// @brief Apply this value to a list of arguments.
        Value Apply(::std::vector<Value> args, const Environment &env);

    private:
        /// @brief Get the string representation of this value when it is a list.
        [[nodiscard]] ::std::string ToStringList() const noexcept;

        Type m_type;
        union
        {
            int i;
            float f;
            Builtin b;
        } m_stack_data{};
        ::std::string m_string_data{};
        ::std::vector<Value> m_list_data{};
        Environment m_lambda_scope{};
    };
} // namespace lisp