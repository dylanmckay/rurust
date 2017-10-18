//! Accesses for standard Ruby classes.

use Value;
use ffi;

// Ruby classes do not follow the same conventions.

macro_rules! define_class {
    ( $class_name:ident => $ffi_value:expr ) => {
        #[allow(non_snake_case)]
        pub fn $class_name() -> Value {
            Value(unsafe { $ffi_value })
        }
    }
}

define_class!(Array => ffi::rb_cArray);
define_class!(BasicObject => ffi::rb_cBasicObject);
define_class!(Binding => ffi::rb_cBinding);
define_class!(Class => ffi::rb_cClass);
define_class!(Complex => ffi::rb_cComplex);
define_class!(Cont => ffi::rb_cCont);
define_class!(Data => ffi::rb_cData);
define_class!(Dir => ffi::rb_cDir);
define_class!(Encoding => ffi::rb_cEncoding);
define_class!(Enumerator => ffi::rb_cEnumerator);
define_class!(FalseClass => ffi::rb_cFalseClass);
define_class!(File => ffi::rb_cFile);
define_class!(Float => ffi::rb_cFloat);
define_class!(Hash => ffi::rb_cHash);
define_class!(IO => ffi::rb_cIO);
define_class!(Integer => ffi::rb_cInteger);
define_class!(Match => ffi::rb_cMatch);
define_class!(Method => ffi::rb_cMethod);
define_class!(Module => ffi::rb_cModule);
define_class!(NameErrorMesg => ffi::rb_cNameErrorMesg);
define_class!(NilClass => ffi::rb_cNilClass);
define_class!(Numeric => ffi::rb_cNumeric);
define_class!(Object => ffi::rb_cObject);
define_class!(Proc => ffi::rb_cProc);
define_class!(Random => ffi::rb_cRandom);
define_class!(Range => ffi::rb_cRange);
define_class!(Rational => ffi::rb_cRational);
define_class!(Regexp => ffi::rb_cRegexp);
define_class!(Stat => ffi::rb_cStat);
define_class!(String => ffi::rb_cString);
define_class!(Struct => ffi::rb_cStruct);
define_class!(Symbol => ffi::rb_cSymbol);
define_class!(Thread => ffi::rb_cThread);
define_class!(Time => ffi::rb_cTime);
define_class!(TrueClass => ffi::rb_cTrueClass);
define_class!(UnboundMethod => ffi::rb_cUnboundMethod);
define_class!(ArgError => ffi::rb_eArgError);
define_class!(EOFError => ffi::rb_eEOFError);
define_class!(EncCompatError => ffi::rb_eEncCompatError);
define_class!(EncodingError => ffi::rb_eEncodingError);
define_class!(Exception => ffi::rb_eException);
define_class!(Fatal => ffi::rb_eFatal);
define_class!(FloatDomainError => ffi::rb_eFloatDomainError);
define_class!(IOError => ffi::rb_eIOError);
define_class!(IndexError => ffi::rb_eIndexError);
define_class!(Interrupt => ffi::rb_eInterrupt);
define_class!(KeyError => ffi::rb_eKeyError);
define_class!(LoadError => ffi::rb_eLoadError);
define_class!(LocalJumpError => ffi::rb_eLocalJumpError);
define_class!(MathDomainError => ffi::rb_eMathDomainError);
define_class!(NameError => ffi::rb_eNameError);
define_class!(NoMemError => ffi::rb_eNoMemError);
define_class!(NoMethodError => ffi::rb_eNoMethodError);
define_class!(NotImpError => ffi::rb_eNotImpError);
define_class!(RangeError => ffi::rb_eRangeError);
define_class!(RegexpError => ffi::rb_eRegexpError);
define_class!(RuntimeError => ffi::rb_eRuntimeError);
define_class!(ScriptError => ffi::rb_eScriptError);
define_class!(SecurityError => ffi::rb_eSecurityError);
define_class!(Signal => ffi::rb_eSignal);
define_class!(StandardError => ffi::rb_eStandardError);
define_class!(StopIteration => ffi::rb_eStopIteration);
define_class!(SyntaxError => ffi::rb_eSyntaxError);
define_class!(SysStackError => ffi::rb_eSysStackError);
define_class!(SystemCallError => ffi::rb_eSystemCallError);
define_class!(SystemExit => ffi::rb_eSystemExit);
define_class!(ThreadError => ffi::rb_eThreadError);
define_class!(TypeError => ffi::rb_eTypeError);
define_class!(ZeroDivError => ffi::rb_eZeroDivError);

