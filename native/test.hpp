template <class T, int sizeN, int sizeM>
 class Bytes {
    T val[sizeN * sizeM];
    public:
	    Bytes(T p1, T p2, T p3) {
	    	val[0] = p1;
            val[1] = p2;
	    	val[2] = p3;
	    };
};

template <class T, int size>
    class BytesC: public Bytes<T, size, 1> {
    public:
        BytesC(T p1, T p2, T p3) : Bytes<T, size, 1>(p1, p2, p3) {}
};