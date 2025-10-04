struct TType {
   c: C,
   b: B,
}

runc main() {
   bind b = "Hi";
   bind re c = Iden::ident();
   if (b == c) {
      c = "Ahoj";
      c = { 5 + 5 }
   }
   return c
}
