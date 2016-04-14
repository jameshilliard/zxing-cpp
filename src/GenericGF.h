#pragma once
#include <vector>
#include "GenericGFPoly.h"

namespace ZXing {

/**
* <p>This class contains utility methods for performing mathematical operations over
* the Galois Fields. Operations use a given primitive polynomial in calculations.</p>
*
* <p>Throughout this package, elements of the GF are represented as an {@code int}
* for convenience and speed (but at the cost of memory).
* </p>
*/
class GenericGF
{
public:
	static const GenericGF& AztecData12();
	static const GenericGF& AztecData10();
	static const GenericGF& AztecData6();
	static const GenericGF& AztecParam();
	static const GenericGF& QRCodeField256();
	static const GenericGF& DataMatrixField256();
	static const GenericGF& AztecData8();
	static const GenericGF& MaxiCodeField64();

	GenericGFPoly zero() const {
		return _zero;
	}

	GenericGFPoly one() const {
		return _one;
	}

	/**
	* @return the monomial representing coefficient * x^degree
	*/
	GenericGFPoly buildMonomial(int degree, int coefficient) const;

	/**
	* Implements both addition and subtraction -- they are the same in GF(size).
	*
	* @return sum/difference of a and b
	*/
	static int AddOrSubtract(int a, int b) {
		return a ^ b;
	}

	/**
	* @return 2 to the power of a in GF(size)
	*/
	int exp(int a) const;

	/**
	* @return base 2 log of a in GF(size)
	*/
	int log(int a) const;

	/**
	* @return multiplicative inverse of a
	*/
	int inverse(int a) const;

	/**
	* @return product of a and b in GF(size)
	*/
	int multiply(int a, int b) const;

	
	int size() const {
		return _size;
	}

	int generatorBase() const {
		return _generatorBase;
	}

private:
	int _size;
	int _primitive;
	int _generatorBase;
	std::vector<int> _expTable;
	std::vector<int> _logTable;
	GenericGFPoly _zero;
	GenericGFPoly _one;

	/**
	* Create a representation of GF(size) using the given primitive polynomial.
	*
	* @param primitive irreducible polynomial whose coefficients are represented by
	*  the bits of an int, where the least-significant bit represents the constant
	*  coefficient
	* @param size the size of the field
	* @param b the factor b in the generator polynomial can be 0- or 1-based
	*  (g(x) = (x+a^b)(x+a^(b+1))...(x+a^(b+2t-1))).
	*  In most cases it should be 1, but for QR code it is 0.
	*/
	GenericGF(int primitive, int size, int b);
};

} // ZXing
