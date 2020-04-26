#include <cstdint>

struct M{};

using my_integer = std::int32_t;

typedef M TM;

using my_m = M;

using alias_alias = my_m;
