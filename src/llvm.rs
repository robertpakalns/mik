use llvm_sys::{analysis::*, core::*, target::*};
use std::ffi::CString;

pub fn compile(log_string: String) {
    unsafe {
        // Initialize LLVM
        LLVM_InitializeNativeTarget();
        LLVM_InitializeNativeAsmPrinter();
        LLVM_InitializeNativeAsmParser();

        let context = LLVMContextCreate();
        let module = LLVMModuleCreateWithNameInContext(
            CString::new("console_log_module").unwrap().as_ptr(),
            context,
        );
        let builder = LLVMCreateBuilderInContext(context);

        let i8_ptr_ty = LLVMPointerType(LLVMInt8TypeInContext(context), 0);

        // Declare printf
        let printf_ty = LLVMFunctionType(
            LLVMInt32TypeInContext(context),
            [i8_ptr_ty].as_mut_ptr(),
            1,
            1,
        );
        let printf_fn =
            LLVMAddFunction(module, CString::new("printf").unwrap().as_ptr(), printf_ty);

        // Main function
        let main_ty = LLVMFunctionType(LLVMInt32TypeInContext(context), std::ptr::null_mut(), 0, 0);
        let main_fn = LLVMAddFunction(module, CString::new("main").unwrap().as_ptr(), main_ty);
        let entry = LLVMAppendBasicBlockInContext(
            context,
            main_fn,
            CString::new("entry").unwrap().as_ptr(),
        );
        LLVMPositionBuilderAtEnd(builder, entry);

        let msg = CString::new(format!("{log_string}\n")).unwrap();
        let msg_ptr =
            LLVMBuildGlobalString(builder, msg.as_ptr(), CString::new("msg").unwrap().as_ptr());

        // Call printf
        let mut args = [msg_ptr];
        LLVMBuildCall2(
            builder,
            printf_ty,
            printf_fn,
            args.as_mut_ptr(),
            args.len() as u32,
            CString::new("").unwrap().as_ptr(),
        );

        // Return 0
        LLVMBuildRet(builder, LLVMConstInt(LLVMInt32TypeInContext(context), 0, 0));

        // Verify & dump module
        let mut error = std::ptr::null_mut();
        LLVMVerifyModule(
            module,
            LLVMVerifierFailureAction::LLVMAbortProcessAction,
            &mut error,
        );
        LLVMDumpModule(module);

        // Clean up
        LLVMDisposeBuilder(builder);
        LLVMDisposeModule(module);
        LLVMContextDispose(context);
    }
}
