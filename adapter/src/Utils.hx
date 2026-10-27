typedef Arguments = {
	cwd:String,
	classPaths:Array<String>,
	?hl:String,
	?env:haxe.DynamicAccess<String>,
	?program:String,
	?args:Array<String>,
	?argsFile:String,
	?port:Int,
	?hotReload:Bool,
	?profileSamples:Int,
	?allowEval:Bool,
	?optimizeDebug:Bool,
	?connectionTimeout:Float
}

class Utils {

	inline public static function toString(value:Int, base:Int):String {
		#if js
		return untyped value.toString(base);
		#end
	}

}
