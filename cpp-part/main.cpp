#include <cstdlib>
#include <iostream>
#include <string>

#include "Foo.hpp"
using namespace rust;

int main()
{
    // ANCHOR: call_rust
    Foo foo(5);
    int res = foo.f(1, 2);
    // ANCHOR_END: call_rust
    if (res == 8) {
        std::cout << "All works fine: " << res << std::endl;
        auto text = std::string(foo.get_text());
        std::cout << "Text: " << text << std::endl;
    }
    else {
        std::cout << "Something really BAD: " << res << std::endl;
        return EXIT_FAILURE;
    }
    return EXIT_SUCCESS;
}
