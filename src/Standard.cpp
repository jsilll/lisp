#include <lisp/Standard.hpp>

#include <algorithm>
#include <iostream>
#include <stdexcept>

#include <lisp/Lexer.hpp>
#include <lisp/Parser.hpp>
#include <lisp/Value.hpp>

namespace lisp::std {
static void
EvalArgs(::std::vector<Value> &args, const Environment &env) {
    for (auto &arg : args) {
        arg = arg.Eval(env);
    }
}

static Value
Lambda(::std::vector<Value> args, const Environment &env) {
    if (args.size() < 2) {
        throw ::std::runtime_error("Too few arguments");
    } else if (args[0].type() != Type::List) {
        throw ::std::runtime_error("First argument must be a list");
    } else {
        return Value(args[0].list_data(), args[1], env);
    }
}

static Value
Map(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 2) {
        throw ::std::runtime_error("Number of arguments must be 2");
    } else {
        ::std::vector<Value> result;
        result.reserve(args[0].list_data().size());
        for (auto &arg : args[0].list_data()) {
            result.push_back(args[1].Apply({arg}, env));
        }
        return Value(result);
    }
}

static Value
Zip(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 2) {
        throw ::std::runtime_error("Number of arguments must be 2");
    } else if (args[0].type() != Type::List || args[1].type() != Type::List) {
        throw ::std::runtime_error("Arguments must be lists");
    } else if (args[0].list_data().size() != args[1].list_data().size()) {
        throw ::std::runtime_error("Lists must be of equal size");
    } else {
        ::std::vector<Value> result;
        result.reserve(args[0].list_data().size());
        for (size_t i = 0; i < args[0].list_data().size(); ++i) {
            result.push_back(
                Value({args[0].list_data()[i], args[1].list_data()[i]}));
        }
        return Value(result);
    }
}

static Value
Fold(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 3) {
        throw ::std::runtime_error("Number of arguments must be 3");
    } else {
        auto result = args[2];
        for (auto &arg : args[0].list_data()) {
            result = args[1].Apply({result, arg}, env);
        }
        return result;
    }
}

static Value
Filter(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 2) {
        throw ::std::runtime_error("Number of arguments must be 2");
    } else {
        ::std::vector<Value> result;
        for (auto &arg : args[0].list_data()) {
            if (args[1].Apply({arg}, env).stack_data().i != 0) {
                result.push_back(arg);
            }
        }
        return Value(result);
    }
}

static Value
If(::std::vector<Value> args, const Environment &env) {
    if (args.size() != 3) {
        throw ::std::runtime_error("Number of arguments must be 3");
    } else {
        auto condition = args[0].Eval(env);
        if (condition.type() == Type::Unit) {
            return args[2].Eval(env);
        } else if (condition.type() == Type::Int) {
            if (condition.stack_data().i == 0) {
                return args[2].Eval(env);
            } else {
                return args[1].Eval(env);
            }
        } else if (condition.type() == Type::Float) {
            if (condition.stack_data().f == 0) {
                return args[2].Eval(env);
            } else {
                return args[1].Eval(env);
            }
        } else if (condition.type() == Type::String) {
            if (condition.string_data().empty()) {
                return args[2].Eval(env);
            } else {
                return args[1].Eval(env);
            }
        } else {
            throw ::std::runtime_error(
                "Condition must be of type int, float, or string");
        }
    }
}

static Value
Let(::std::vector<Value> args, const Environment &env) {
    if (args.size() != 2) {
        throw ::std::runtime_error("Number of arguments must be 2");
    } else {
        if (args[0].type() != Type::List) {
            throw ::std::runtime_error("First argument must be a list");
        } else if (args[0].list_data().size() != 2) {
            throw ::std::runtime_error(
                "First argument must be a list of size 2");
        } else if (args[0].list_data()[0].type() != Type::Atom) {
            throw ::std::runtime_error("Binding name must be an atom");
        } else {
            const auto &name = args[0].list_data()[0].string_data();
            auto value = args[0].list_data()[1].Eval(env);
            Environment new_env;
            new_env.SetParent(env);
            new_env.Set(::std::move(name), ::std::move(value));
            return args[1].Eval(new_env);
        }
    }
}

static Value
Plus(const Value &lhs, const Value &rhs) {
    if (lhs.type() == Type::Int) {
        if (rhs.type() == Type::Int) {
            return Value(lhs.stack_data().i + rhs.stack_data().i);
        } else if (rhs.type() == Type::Float) {
            return Value(lhs.stack_data().i + rhs.stack_data().f);
        } else {
            throw ::std::runtime_error(
                "Cannot add numeric and non-numeric values");
        }
    } else if (lhs.type() == Type::Float) {
        if (rhs.type() == Type::Int) {
            return Value(lhs.stack_data().f + rhs.stack_data().i);
        } else if (rhs.type() == Type::Float) {
            return Value(lhs.stack_data().f + rhs.stack_data().f);
        } else {
            throw ::std::runtime_error(
                "Cannot add numeric and non-numeric values");
        }
    } else {
        throw ::std::runtime_error("Cannot add non-numeric values");
    }
}

static Value
Plus(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.empty()) {
        return Value(0);
    } else {
        Value sum = Plus(args[0], Value(0));
        for (auto it = args.begin() + 1; it != args.end(); ++it) {
            sum = Plus(sum, *it);
        }
        return sum;
    }
}

static Value
Minus(const Value &lhs, const Value &rhs) {
    if (lhs.type() == Type::Int) {
        if (rhs.type() == Type::Int) {
            return Value(lhs.stack_data().i - rhs.stack_data().i);
        } else if (rhs.type() == Type::Float) {
            return Value(lhs.stack_data().i - rhs.stack_data().f);
        } else {
            throw ::std::runtime_error(
                "Cannot subtract numeric and non-numeric values");
        }
    } else if (lhs.type() == Type::Float) {
        if (rhs.type() == Type::Int) {
            return Value(lhs.stack_data().f - rhs.stack_data().i);
        } else if (rhs.type() == Type::Float) {
            return Value(lhs.stack_data().f - rhs.stack_data().f);
        } else {
            throw ::std::runtime_error(
                "Cannot subtract numeric and non-numeric values");
        }
    } else {
        throw ::std::runtime_error("Cannot subtract non-numeric values");
    }
}

static Value
Minus(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 2) {
        throw ::std::runtime_error("Number of arguments must be 2");
    } else {
        return Minus(args[0], args[1]);
    }
}

static Value
Times(const Value &lhs, const Value &rhs) {
    if (lhs.type() == Type::Int) {
        if (rhs.type() == Type::Int) {
            return Value(lhs.stack_data().i * rhs.stack_data().i);
        } else if (rhs.type() == Type::Float) {
            return Value(lhs.stack_data().i * rhs.stack_data().f);
        } else {
            throw ::std::runtime_error(
                "Cannot multiply numeric and non-numeric values");
        }
    } else if (lhs.type() == Type::Float) {
        if (rhs.type() == Type::Int) {
            return Value(lhs.stack_data().f * rhs.stack_data().i);
        } else if (rhs.type() == Type::Float) {
            return Value(lhs.stack_data().f * rhs.stack_data().f);
        } else {
            throw ::std::runtime_error(
                "Cannot multiply numeric and non-numeric values");
        }
    } else {
        throw ::std::runtime_error("Cannot multiply non-numeric values");
    }
}

static Value
Times(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.empty()) {
        return Value(1);
    } else {
        Value product = Times(args[0], Value(1));
        for (auto it = args.begin() + 1; it != args.end(); ++it) {
            product = Times(product, *it);
        }
        return product;
    }
}

static Value
Divide(const Value &lhs, const Value &rhs) {
    if (rhs.type() == Type::Int) {
        if (rhs.stack_data().i == 0) {
            throw ::std::runtime_error("Cannot divide by zero");
        } else if (lhs.type() == Type::Int) {
            return Value(lhs.stack_data().i / rhs.stack_data().i);
        } else if (lhs.type() == Type::Float) {
            return Value(lhs.stack_data().f / rhs.stack_data().i);
        } else {
            throw ::std::runtime_error(
                "Cannot divide numeric and non-numeric values");
        }
    } else if (rhs.type() == Type::Float) {
        if (rhs.stack_data().f == 0) {
            throw ::std::runtime_error("Cannot divide by zero");
        } else if (lhs.type() == Type::Int) {
            return Value(lhs.stack_data().i / rhs.stack_data().f);
        } else if (lhs.type() == Type::Float) {
            return Value(lhs.stack_data().f / rhs.stack_data().f);
        } else {
            throw ::std::runtime_error(
                "Cannot divide numeric and non-numeric values");
        }
    } else {
        throw ::std::runtime_error("Cannot divide non-numeric values");
    }
}

static Value
Divide(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 2) {
        throw ::std::runtime_error("Number of arguments must be 2");
    } else {
        return Divide(args[0], args[1]);
    }
}

static Value
Equals(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 2) {
        throw ::std::runtime_error("Number of arguments must be 2");
    } else if (args[0].type() == Type::Int) {
        if (args[1].type() == Type::Int) {
            return args[0].stack_data().i == args[1].stack_data().i ? Value(1)
                                                                    : Value(0);
        } else if (args[1].type() == Type::Float) {
            return args[0].stack_data().i == args[1].stack_data().f ? Value(1)
                                                                    : Value(0);
        } else {
            throw ::std::runtime_error(
                "Cannot compare numeric and non-numeric values");
        }
    } else if (args[0].type() == Type::Float) {
        if (args[1].type() == Type::Int) {
            return args[0].stack_data().f == args[1].stack_data().i ? Value(1)
                                                                    : Value(0);
        } else if (args[1].type() == Type::Float) {
            return args[0].stack_data().f == args[1].stack_data().f ? Value(1)
                                                                    : Value(0);
        } else {
            throw ::std::runtime_error(
                "Cannot compare numeric and non-numeric values");
        }
    } else if (args[0].type() == Type::String) {
        if (args[1].type() == Type::String) {
            return args[0].string_data() == args[1].string_data() ? Value(1)
                                                                  : Value(0);
        } else {
            throw ::std::runtime_error(
                "Cannot compare string and non-string values");
        }
    } else if (args[0].type() == Type::Unit) {
        return args[1].type() == Type::Unit ? Value(1) : Value(0);
    } else {
        throw ::std::runtime_error(
            "Only numeric and string values can be compared");
    }
}

static Value
NotEquals(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 2) {
        throw ::std::runtime_error("Number of arguments must be 2");
    } else if (args[0].type() == Type::Int) {
        if (args[1].type() == Type::Int) {
            return args[0].stack_data().i != args[1].stack_data().i ? Value(1)
                                                                    : Value(0);
        } else if (args[1].type() == Type::Float) {
            return args[0].stack_data().i != args[1].stack_data().f ? Value(1)
                                                                    : Value(0);
        } else {
            throw ::std::runtime_error(
                "Cannot compare numeric and non-numeric values");
        }
    } else if (args[0].type() == Type::Float) {
        if (args[1].type() == Type::Int) {
            return args[0].stack_data().f != args[1].stack_data().i ? Value(1)
                                                                    : Value(0);
        } else if (args[1].type() == Type::Float) {
            return args[0].stack_data().f != args[1].stack_data().f ? Value(1)
                                                                    : Value(0);
        } else {
            throw ::std::runtime_error(
                "Cannot compare numeric and non-numeric values");
        }
    } else if (args[0].type() == Type::String) {
        if (args[1].type() == Type::String) {
            return args[0].string_data() != args[1].string_data() ? Value(1)
                                                                  : Value(0);
        } else {
            throw ::std::runtime_error(
                "Cannot compare string and non-string values");
        }
    } else if (args[0].type() == Type::Unit) {
        return args[1].type() == Type::Unit ? Value(0) : Value(1);
    } else {
        throw ::std::runtime_error(
            "Only numeric and string values can be compared");
    }
}

static Value
Less(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 2) {
        throw ::std::runtime_error("Number of arguments must be 2");
    } else if (args[0].type() == Type::Int) {
        if (args[1].type() == Type::Int) {
            return args[0].stack_data().i < args[1].stack_data().i ? Value(1)
                                                                   : Value(0);
        } else if (args[1].type() == Type::Float) {
            return args[0].stack_data().i < args[1].stack_data().f ? Value(1)
                                                                   : Value(0);
        } else {
            throw ::std::runtime_error(
                "Cannot compare numeric and non-numeric values");
        }
    } else if (args[0].type() == Type::Float) {
        if (args[1].type() == Type::Int) {
            return args[0].stack_data().f < args[1].stack_data().i ? Value(1)
                                                                   : Value(0);
        } else if (args[1].type() == Type::Float) {
            return args[0].stack_data().f < args[1].stack_data().f ? Value(1)
                                                                   : Value(0);
        } else {
            throw ::std::runtime_error(
                "Cannot compare numeric and non-numeric values");
        }
    } else {
        throw ::std::runtime_error("Cannot compare non-numeric values");
    }
}

static Value
Greater(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 2) {
        throw ::std::runtime_error("Number of arguments must be 2");
    } else if (args[0].type() == Type::Int) {
        if (args[1].type() == Type::Int) {
            return args[0].stack_data().i > args[1].stack_data().i ? Value(1)
                                                                   : Value(0);
        } else if (args[1].type() == Type::Float) {
            return args[0].stack_data().i > args[1].stack_data().f ? Value(1)
                                                                   : Value(0);
        } else {
            throw ::std::runtime_error(
                "Cannot compare numeric and non-numeric values");
        }
    } else if (args[0].type() == Type::Float) {
        if (args[1].type() == Type::Int) {
            return args[0].stack_data().f > args[1].stack_data().i ? Value(1)
                                                                   : Value(0);
        } else if (args[1].type() == Type::Float) {
            return args[0].stack_data().f > args[1].stack_data().f ? Value(1)
                                                                   : Value(0);
        } else {
            throw ::std::runtime_error(
                "Cannot compare numeric and non-numeric values");
        }
    } else {
        throw ::std::runtime_error("Cannot compare non-numeric values");
    }
}

static Value
LessEquals(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 2) {
        throw ::std::runtime_error("Number of arguments must be 2");
    } else if (args[0].type() == Type::Int) {
        if (args[1].type() == Type::Int) {
            return args[0].stack_data().i <= args[1].stack_data().i ? Value(1)
                                                                    : Value(0);
        } else if (args[1].type() == Type::Float) {
            return args[0].stack_data().i <= args[1].stack_data().f ? Value(1)
                                                                    : Value(0);
        } else {
            throw ::std::runtime_error(
                "Cannot compare numeric and non-numeric values");
        }
    } else if (args[0].type() == Type::Float) {
        if (args[1].type() == Type::Int) {
            return args[0].stack_data().f <= args[1].stack_data().i ? Value(1)
                                                                    : Value(0);
        } else if (args[1].type() == Type::Float) {
            return args[0].stack_data().f <= args[1].stack_data().f ? Value(1)
                                                                    : Value(0);
        } else {
            throw ::std::runtime_error(
                "Cannot compare numeric and non-numeric values");
        }
    } else {
        throw ::std::runtime_error("Cannot compare non-numeric values");
    }
}

static Value
GreaterEquals(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 2) {
        throw ::std::runtime_error("Number of arguments must be 2");
    } else if (args[0].type() == Type::Int) {
        if (args[1].type() == Type::Int) {
            return args[0].stack_data().i >= args[1].stack_data().i ? Value(1)
                                                                    : Value(0);
        } else if (args[1].type() == Type::Float) {
            return args[0].stack_data().i >= args[1].stack_data().f ? Value(1)
                                                                    : Value(0);
        } else {
            throw ::std::runtime_error(
                "Cannot compare numeric and non-numeric values");
        }
    } else if (args[0].type() == Type::Float) {
        if (args[1].type() == Type::Int) {
            return args[0].stack_data().f >= args[1].stack_data().i ? Value(1)
                                                                    : Value(0);
        } else if (args[1].type() == Type::Float) {
            return args[0].stack_data().f >= args[1].stack_data().f ? Value(1)
                                                                    : Value(0);
        } else {
            throw ::std::runtime_error(
                "Cannot compare numeric and non-numeric values");
        }
    } else {
        throw ::std::runtime_error("Cannot compare non-numeric values");
    }
}

static Value
Abs(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 1) {
        throw ::std::runtime_error("Number of arguments must be 1");
    } else if (args[0].type() == Type::Int) {
        return Value(::std::abs(args[0].stack_data().i));
    } else if (args[0].type() == Type::Float) {
        return Value(::std::abs(args[0].stack_data().f));
    } else {
        throw ::std::runtime_error("Argument must be an int or float");
    }
}

static Value
Odd(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 1) {
        throw ::std::runtime_error("Number of arguments must be 1");
    } else if (args[0].type() == Type::Int) {
        return args[0].stack_data().i % 2 != 0 ? Value(1) : Value(0);
    } else if (args[0].type() == Type::Float) {
        return static_cast<int>(args[0].stack_data().f) % 2 != 0 ? Value(1)
                                                                 : Value(0);
    } else {
        throw ::std::runtime_error("Argument must be an int or float");
    }
}

static Value
Even(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 1) {
        throw ::std::runtime_error("Number of arguments must be 1");
    } else if (args[0].type() == Type::Int) {
        return args[0].stack_data().i % 2 == 0 ? Value(1) : Value(0);
    } else if (args[0].type() == Type::Float) {
        return static_cast<int>(args[0].stack_data().f) % 2 == 0 ? Value(1)
                                                                 : Value(0);
    } else {
        throw ::std::runtime_error("Argument must be an int or a float");
    }
}

static Value
Upper(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 1) {
        throw ::std::runtime_error("Number of arguments must be 1");
    } else if (args[0].type() != Type::String) {
        throw ::std::runtime_error("Argument must be a string");
    } else {
        auto result = args[0].string_data();
        ::std::transform(result.begin(), result.end(), result.begin(),
                         ::toupper);
        return Value(result, Type::String);
    }
}

static Value
Lower(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 1) {
        throw ::std::runtime_error("Number of arguments must be 1");
    } else if (args[0].type() != Type::String) {
        throw ::std::runtime_error("Argument must be a string");
    } else {
        auto result = args[0].string_data();
        ::std::transform(result.begin(), result.end(), result.begin(),
                         ::tolower);
        return Value(result, Type::String);
    }
}

static Value
ToStr(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 1) {
        throw ::std::runtime_error("Number of arguments must be 1");
    } else {
        return Value(args[0].ToString(), Type::String);
    }
}

static Value
Head(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 1) {
        throw ::std::runtime_error("Number of arguments must be 1");
    } else if (args[0].type() != Type::List) {
        throw ::std::runtime_error("Argument must be a list");
    } else {
        return args[0].list_data().empty() ? Value() : args[0].list_data()[0];
    }
}

static Value
Tail(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 1) {
        throw ::std::runtime_error("Number of arguments must be 1");
    } else if (args[0].type() != Type::List) {
        throw ::std::runtime_error("Argument must be a list");
    } else {
        auto result = args[0].list_data();
        result.erase(result.begin());
        return result.empty() ? Value() : Value(result);
    }
}

static Value
Range(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 1) {
        throw ::std::runtime_error("Number of arguments must be 1");
    } else if (args[0].type() != Type::Int) {
        throw ::std::runtime_error("Argument must be an int");
    } else {
        ::std::vector<Value> result;
        result.reserve(args[0].stack_data().i);
        for (int i = 0; i < args[0].stack_data().i; ++i) {
            result.push_back(Value(i));
        }
        return Value(result);
    }
}

static Value
Print(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 1) {
        throw ::std::runtime_error("Number of arguments must be 2");
    } else if (args[0].type() != Type::String) {
        throw ::std::runtime_error("Argument must be a string");
    } else {
        ::std::cout << args[0].string_data() << "\n";
        return Value();
    }
}

static Value
Input(::std::vector<Value> args, [[maybe_unused]] const Environment &env) {
    if (args.size() != 0) {
        throw ::std::runtime_error("Number of arguments must be 0");
    } else {
        ::std::string input;
        ::std::getline(::std::cin, input);
        return Value(::std::move(input), Type::String);
    }
}

static Value
Parse(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 1) {
        throw ::std::runtime_error("Number of arguments must be 1");
    } else {
        auto lexer = Lexer(args[0].string_data());
        auto parser = Parser(::std::move(lexer));
        auto result = parser.Parse();
        return result;
    }
}

static Value
Eval(::std::vector<Value> args, const Environment &env) {
    EvalArgs(args, env);

    if (args.size() != 1) {
        throw ::std::runtime_error("Number of arguments must be 1");
    } else if (args[0].type() != Type::List) {
        throw ::std::runtime_error("Argument must be a list");
    } else {
        return args[0].Eval(env);
    }
}

void
Register(Environment &env) noexcept {
    // Base
    env.Set("nil", Value());
    env.Set("lambda", Value("lambda", Lambda));

    // Functional let
    env.Set("let", Value("let", Let));

    // Functional abstractions
    env.Set("map", Value("map", Map));
    env.Set("zip", Value("zip", Zip));
    env.Set("fold", Value("fold", Fold));
    env.Set("filter", Value("filter", Filter));

    // If expression
    env.Set("if", Value("if", If));

    // Arithmetic
    env.Set("+", Value("+", Plus));
    env.Set("-", Value("-", Minus));
    env.Set("*", Value("*", Times));
    env.Set("/", Value("/", Divide));

    // Comparison
    env.Set("==", Value("==", Equals));
    env.Set("!=", Value("!=", NotEquals));
    env.Set("<", Value("<", Less));
    env.Set(">", Value(">", Greater));
    env.Set("<=", Value("<=", LessEquals));
    env.Set(">=", Value(">=", GreaterEquals));

    // Number operations
    env.Set("abs", Value("abs", Abs));
    env.Set("odd?", Value("odd?", Odd));
    env.Set("even?", Value("even?", Even));

    // String operations
    env.Set("upper", Value("upper", Upper));
    env.Set("lower", Value("lower", Lower));
    env.Set("to_str", Value("to_str", ToStr));

    // List operations
    env.Set("head", Value("head", Head));
    env.Set("tail", Value("tail", Tail));

    // Iteration
    env.Set("range", Value("range", Range));

    // Stdio
    env.Set("print", Value("print", Print));
    env.Set("input", Value("input", Input));

    // Meta Circular
    env.Set("parse", Value("parse", Parse));
    env.Set("eval", Value("eval", Eval));
}
}   // namespace lisp::std