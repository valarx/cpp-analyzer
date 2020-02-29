#ifndef CLASS_H
#define CLASS_H

namespace my_namespace {
class MyTestClass {
  struct PrivateStruct {};
  int field;

public:
  float field1;
  MyTestClass();
  MyTestClass(const MyTestClass &);
  MyTestClass &operator=(const MyTestClass &);

protected:
  bool field3;
  void test_method();
  MyTestClass(MyTestClass &&);
};
} // namespace my_namespace
#endif
