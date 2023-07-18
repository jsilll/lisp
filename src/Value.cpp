#include "Value.hpp"

#include <algorithm>
#include <stdexcept>

namespace lisp
{
    Value::Value(std::string s, Type t)
        : m_type(t),
          m_string_data(std::move(s))
    {
        if (t != Type::String and t != Type::Atom)
        {
            throw std::runtime_error("Value must be a string or an atom");
        }
    }

    Value::Value(std::vector<Value> args, Value body, const Environment &scope) noexcept
        : m_type(Type::Lambda)
    {
        for (const auto &atom : body.GetAtoms())
        {
            if (const auto bind = scope.Get(atom))
            {
                m_lambda_scope.Set(atom, *bind);
            }
        }
        m_list_data.emplace_back(std::move(args));
        m_list_data.emplace_back(std::move(body));
    }

    Value Value::Quote() const noexcept
    {
        Value v;
        v.m_type = Type::Quote;
        v.m_list_data.emplace_back(*this);
        return v;
    }

    std::vector<std::string> Value::GetAtoms() const noexcept
    {
        switch (m_type)
        {
        case Type::Quote:
            return m_list_data[0].GetAtoms();
        case Type::Atom:
            return {m_string_data};
        case Type::Lambda:
            return m_list_data[1].GetAtoms();
        default:
            if (m_type == Type::List)
            {
                std::vector<std::string> atoms;
                std::for_each(m_list_data.begin(), m_list_data.end(), [&atoms](const Value &v)
                              {
                    auto v_atoms = v.GetAtoms();
                    atoms.insert(atoms.end(), v_atoms.begin(), v_atoms.end()); });
                return atoms;
            }
            else
            {
                return {};
            }
        }
    }

    std::string Value::ToString() const noexcept
    {
        switch (m_type)
        {
        case Type::Unit:
            return "unit";
        case Type::Int:
            return std::to_string(m_stack_data.i) + " : int";
        case Type::Float:
            return std::to_string(m_stack_data.f) + " : float";
        case Type::String:
            return "\"" + m_string_data + "\" : str";
        case Type::Atom:
            return m_string_data + " : atom";
        case Type::Builtin:
            return "<builtin>";
        case Type::Lambda:
            return "<lambda>";
        case Type::Quote:
            return "'" + m_list_data[0].ToString();
        case Type::List:
            return "(" + ToStringList() + ")";
        }
    }

    Value Value::Eval([[maybe_unused]] Environment &env)
    {
        // TODO: Implement
        return {};
    }

    Value Value::Apply([[maybe_unused]] const std::vector<Value> args, [[maybe_unused]] Environment &env)
    {
        // TODO: Implement
        return {};
    }

    std::string Value::ToStringList() const noexcept
    {
        if (m_list_data.empty())
        {
            return "";
        }
        else
        {
            std::string s;
            for (const auto &v : m_list_data)
            {
                s += v.ToString() + " ";
            }
            s.pop_back();
            return s;
        }
    }

} // namespace lisp
