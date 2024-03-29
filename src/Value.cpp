#include <lisp/Value.hpp>

#include <algorithm>
#include <cstdint>
#include <stdexcept>

namespace lisp {
Value::Value(std::string s, Type t) : m_type(t), m_string_data(std::move(s)) {
    if (t != Type::String and t != Type::Atom) {
        throw std::runtime_error("Value must be a string or an atom");
    }
}

Value::Value(std::vector<Value> args, Value body,
             const Environment &scope) noexcept
    : m_type(Type::Lambda) {
    for (const auto &atom : body.GetAtoms()) {
        if (const auto bind = scope.Get(atom)) {
            m_lambda_scope.Set(atom, std::move(*bind));
        }
    }
    m_list_data.emplace_back(std::move(args));
    m_list_data.emplace_back(std::move(body));
}

Value
Value::Quote() const noexcept {
    Value v;
    v.m_type = Type::Quote;
    v.m_list_data.emplace_back(*this);
    return v;
}

std::vector<std::string>
Value::GetAtoms() const noexcept {
    switch (m_type) {
    case Type::Quote:
        return m_list_data[0].GetAtoms();
    case Type::Atom:
        return {m_string_data};
    case Type::Lambda:
        return m_list_data[1].GetAtoms();
    default:
        if (m_type == Type::List) {
            std::vector<std::string> atoms;
            std::for_each(m_list_data.begin(), m_list_data.end(),
                          [&atoms](const Value &v) {
                              auto v_atoms = v.GetAtoms();
                              atoms.insert(atoms.end(), v_atoms.begin(),
                                           v_atoms.end());
                          });
            return atoms;
        } else {
            return {};
        }
    }
}

std::string
Value::ToString() const noexcept {
    switch (m_type) {
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
    case Type::Lambda:
        return "<lambda>";
    case Type::Builtin:
        return "<" + m_string_data + " at " +
               ::std::to_string(
                   reinterpret_cast<std::uintptr_t>(m_stack_data.b)) +
               ">";
    case Type::Quote:
        return "'" + m_list_data[0].ToString();
    case Type::List:
        return "(" + ToStringList() + ")";
    }
}

Value
Value::Eval(const Environment &env) const {
    if (m_type == Type::Quote) {
        return m_list_data[0];
    } else if (m_type == Type::Atom) {
        const auto value = env.Get(m_string_data);
        if (value) {
            return *value;
        } else {
            throw std::runtime_error("Symbol not found");
        }
    } else if (m_type == Type::List) {
        if (m_list_data.empty()) {
            return Value();
        } else {
            auto function = m_list_data[0].Eval(env);
            std::vector<Value> args(m_list_data.begin() + 1, m_list_data.end());
            if (function.m_type == Type::Builtin) {
                return function.Apply(std::move(args), env);
            } else if (function.m_type == Type::Lambda) {
                for (std::size_t i = 0; i < args.size(); ++i) {
                    args[i] = args[i].Eval(env);
                }
                return function.Apply(std::move(args), env);
            } else {
                throw std::runtime_error("Cannot apply a non-function value");
            }
        }
    } else {
        return *this;
    }
}

Value
Value::Apply(const std::vector<Value> args, const Environment &env) {

    if (m_type == Type::Lambda) {
        if (m_list_data[0].m_list_data.size() < args.size()) {
            throw std::runtime_error("Too many arguments");
        } else if (m_list_data[0].m_list_data.size() > args.size()) {
            throw std::runtime_error("Too few arguments");
        } else {
            m_lambda_scope.SetParent(env);
            for (std::size_t i = 0; i < m_list_data[0].m_list_data.size();
                 ++i) {
                if (m_list_data[0].m_list_data[i].m_type != Type::Atom) {
                    throw std::runtime_error("Invalid lambda");
                } else {
                    m_lambda_scope.Set(
                        m_list_data[0].m_list_data[i].m_string_data, args[i]);
                }
            }
            return m_list_data[1].Eval(m_lambda_scope);
        }
    } else if (m_type == Type::Builtin) {
        return (m_stack_data.b)(std::move(args), env);
    } else {
        throw std::runtime_error("Cannot apply a non-function value");
    }
}

std::string
Value::ToStringList() const noexcept {
    if (m_list_data.empty()) {
        return "";
    } else {
        std::string s;
        for (const auto &v : m_list_data) {
            s += v.ToString() + " ";
        }
        s.pop_back();
        return s;
    }
}

}   // namespace lisp
