class Main {

	static function main() {
		var value = twice(21);
		trace(value);
		throw "Example exception: the debugger should stop here";
	}

	static function twice(value:Int):Int {
		return value * 2;
	}

}
