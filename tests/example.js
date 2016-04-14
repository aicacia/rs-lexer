function fac(x) {
    if (x <= 0x01) {
        return 1.0;
    } else {
        return x * fac(x - 1);
    }
}
