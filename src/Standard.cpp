#include <lisp/Standard.hpp>

#include <lisp/Value.hpp>

namespace lisp::std
{
    static void EvalArgs(::std::vector<Value> &args, const Environment &env) noexcept
    {
        for (auto &arg : args)
        {
            arg = arg.Eval(env);
        }
    }

    static Value Plus(::std::vector<Value> args, const Environment &env) noexcept
    {
        EvalArgs(args, env);
        if (args.empty())
        {
            return Value(0);
        }
        else
        {
            // TODO: Implement C++ inter-op
            return Value(0);
        }
    }

    static Value Minus(::std::vector<Value> args, const Environment &env) noexcept
    {
        EvalArgs(args, env);
        if (args.empty())
        {
            return Value(0);
        }
        else
        {
            // TODO: Implement C++ inter-op
            return Value(0);
        }
    }

    static Value Times(::std::vector<Value> args, const Environment &env) noexcept
    {
        EvalArgs(args, env);
        if (args.empty())
        {
            return Value(1);
        }
        else
        {
            // TODO: Implement C++ inter-op
            return Value(1);
        }
    }

    static Value Divide(::std::vector<Value> args, const Environment &env) noexcept
    {
        EvalArgs(args, env);
        if (args.empty())
        {
            return Value(1);
        }
        else
        {
            // TODO: Implement C++ inter-op
            return Value(1);
        }
    }

    void Register(Environment &env) noexcept
    {
        env.Set("nil", Value());
        env.Set("+", Value("+", Plus));
        env.Set("-", Value("-", Minus));
        env.Set("*", Value("*", Times));
        env.Set("/", Value("/", Divide));
    }
} // namespace lisp::std
