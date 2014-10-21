#![feature(macro_rules)]

extern crate libc;
extern crate core;

use core::cell::UnsafeCell;
use core::kinds::marker::InvariantLifetime;
use libc::{c_int, c_void, size_t};

pub const FUNC_MAX_ARGS: c_int = 100;

type PGFunction = extern fn(FunctionCallInfo) -> Datum;
type fmNodePtr = *mut c_void;
type fmAggrefPtr = *mut c_void;

/// A trait that is implemented for all Postgres-compatible data types.
trait PgType {}

impl PgType for i16 {}
impl PgType for bool {}

#[allow(dead_code)]
extern {
    static no_such_variable: c_int;
    pub fn pg_malloc(size: size_t) -> *mut c_void;
    pub fn pg_free(ptr: *mut c_void);
}

/// Variable-length datatypes all share this header.
/// The length is encoded as a `char` in C, which is equivalent
/// to `i8`.
///
/// Postgres uses a workaround to have a variable-length data field in that
/// they use a fixed-size, single-element array to fit the data. They simply
/// allocate anough room for the real size and use the array as a pointer. This
/// only works because the compiler doesn't verify the integrity of the size of the
/// array. We'll simply work with a mutable pointer, instead.
///
/// https://github.com/postgres/postgres/blob/master/src/include/c.h#L391
#[repr(C)]
pub struct Varlena {
    len: [i8, ..4],
    data: *mut i8
}

#[repr(C)]
pub struct Text {
    p: Varlena
}

#[repr(C)]
pub struct BpChar {
    p: Varlena
}

#[repr(C)]
pub struct VarChar {
    p: Varlena
}

#[repr(C)]
pub struct Bytea {
    p: Varlena
}

#[repr(C)]
pub struct PgVector<T> {
    len: i32,
    ndim: c_int,
    data_offset: i32,
    elemtype: c_void,
    dim1: c_int,
    lbound1: c_int,
    values: [T, ..1]
}

impl<T> PgVector<T>
    where T: PgType {

}

#[repr(C)]
pub enum NodeTag {
    T_Invalid = 0,
    T_IndexInfo = 10,
    T_ExprContext,
    T_ProjectionInfo,
    T_JunkFilter,
    T_ResultRelInfo,
    T_EState,
    T_TupleTableSlot,

    T_Plan = 100,
    T_Result,
    T_ModifyTable,
    T_Append,
    T_MergeAppend,
    T_RecursiveUnion,
    T_BitmapAnd,
    T_BitmapOr,
    T_Scan,
    T_SeqScan,
    T_IndexScan,
    T_IndexOnlyScan,
    T_BitmapIndexScan,
    T_BitmapHeapScan,
    T_TidScan,
    T_SubqueryScan,
    T_FunctionScan,
    T_ValuesScan,
    T_CteScan,
    T_WorkTableScan,
    T_ForeignScan,
    T_Join,
    T_NestLoop,
    T_MergeJoin,
    T_HashJoin,
    T_Material,
    T_Sort,
    T_Group,
    T_Agg,
    T_WindowAgg,
    T_Unique,
    T_Hash,
    T_SetOp,
    T_LockRows,
    T_Limit,
    T_NestLoopParam,
    T_PlanRowMark,
    T_PlanInvalItem,
    T_PlanState = 200,
    T_ResultState,
    T_ModifyTableState,
    T_AppendState,
    T_MergeAppendState,
    T_RecursiveUnionState,
    T_BitmapAndState,
    T_BitmapOrState,
    T_ScanState,
    T_SeqScanState,
    T_IndexScanState,
    T_IndexOnlyScanState,
    T_BitmapIndexScanState,
    T_BitmapHeapScanState,
    T_TidScanState,
    T_SubqueryScanState,
    T_FunctionScanState,
    T_ValuesScanState,
    T_CteScanState,
    T_WorkTableScanState,
    T_ForeignScanState,
    T_JoinState,
    T_NestLoopState,
    T_MergeJoinState,
    T_HashJoinState,
    T_MaterialState,
    T_SortState,
    T_GroupState,
    T_AggState,
    T_WindowAggState,
    T_UniqueState,
    T_HashState,
    T_SetOpState,
    T_LockRowsState,
    T_LimitState,
    T_Alias = 300,
    T_RangeVar,
    T_Expr,
    T_Var,
    T_Const,
    T_Param,
    T_Aggref,
    T_WindowFunc,
    T_ArrayRef,
    T_FuncExpr,
    T_NamedArgExpr,
    T_OpExpr,
    T_DistinctExpr,
    T_NullIfExpr,
    T_ScalarArrayOpExpr,
    T_BoolExpr,
    T_SubLink,
    T_SubPlan,
    T_AlternativeSubPlan,
    T_FieldSelect,
    T_FieldStore,
    T_RelabelType,
    T_CoerceViaIO,
    T_ArrayCoerceExpr,
    T_ConvertRowtypeExpr,
    T_CollateExpr,
    T_CaseExpr,
    T_CaseWhen,
    T_CaseTestExpr,
    T_ArrayExpr,
    T_RowExpr,
    T_RowCompareExpr,
    T_CoalesceExpr,
    T_MinMaxExpr,
    T_XmlExpr,
    T_NullTest,
    T_BooleanTest,
    T_CoerceToDomain,
    T_CoerceToDomainValue,
    T_SetToDefault,
    T_CurrentOfExpr,
    T_TargetEntry,
    T_RangeTblRef,
    T_JoinExpr,
    T_FromExpr,
    T_IntoClause,
    T_ExprState = 400,
    T_GenericExprState,
    T_WholeRowVarExprState,
    T_AggrefExprState,
    T_WindowFuncExprState,
    T_ArrayRefExprState,
    T_FuncExprState,
    T_ScalarArrayOpExprState,
    T_BoolExprState,
    T_SubPlanState,
    T_AlternativeSubPlanState,
    T_FieldSelectState,
    T_FieldStoreState,
    T_CoerceViaIOState,
    T_ArrayCoerceExprState,
    T_ConvertRowtypeExprState,
    T_CaseExprState,
    T_CaseWhenState,
    T_ArrayExprState,
    T_RowExprState,
    T_RowCompareExprState,
    T_CoalesceExprState,
    T_MinMaxExprState,
    T_XmlExprState,
    T_NullTestState,
    T_CoerceToDomainState,
    T_DomainConstraintState,
    T_PlannerInfo = 500,
    T_PlannerGlobal,
    T_RelOptInfo,
    T_IndexOptInfo,
    T_ParamPathInfo,
    T_Path,
    T_IndexPath,
    T_BitmapHeapPath,
    T_BitmapAndPath,
    T_BitmapOrPath,
    T_NestPath,
    T_MergePath,
    T_HashPath,
    T_TidPath,
    T_ForeignPath,
    T_AppendPath,
    T_MergeAppendPath,
    T_ResultPath,
    T_MaterialPath,
    T_UniquePath,
    T_EquivalenceClass,
    T_EquivalenceMember,
    T_PathKey,
    T_RestrictInfo,
    T_PlaceHolderVar,
    T_SpecialJoinInfo,
    T_LateralJoinInfo,
    T_AppendRelInfo,
    T_PlaceHolderInfo,
    T_MinMaxAggInfo,
    T_PlannerParamItem,
    T_MemoryContext = 600,
    T_AllocSetContext,
    T_Value = 650,
    T_Integer,
    T_Float,
    T_String,
    T_BitString,
    T_Null,
    T_List,
    T_IntList,
    T_OidList,
    T_Query = 700,
    T_PlannedStmt,
    T_InsertStmt,
    T_DeleteStmt,
    T_UpdateStmt,
    T_SelectStmt,
    T_AlterTableStmt,
    T_AlterTableCmd,
    T_AlterDomainStmt,
    T_SetOperationStmt,
    T_GrantStmt,
    T_GrantRoleStmt,
    T_AlterDefaultPrivilegesStmt,
    T_ClosePortalStmt,
    T_ClusterStmt,
    T_CopyStmt,
    T_CreateStmt,
    T_DefineStmt,
    T_DropStmt,
    T_TruncateStmt,
    T_CommentStmt,
    T_FetchStmt,
    T_IndexStmt,
    T_CreateFunctionStmt,
    T_AlterFunctionStmt,
    T_DoStmt,
    T_RenameStmt,
    T_RuleStmt,
    T_NotifyStmt,
    T_ListenStmt,
    T_UnlistenStmt,
    T_TransactionStmt,
    T_ViewStmt,
    T_LoadStmt,
    T_CreateDomainStmt,
    T_CreatedbStmt,
    T_DropdbStmt,
    T_VacuumStmt,
    T_ExplainStmt,
    T_CreateTableAsStmt,
    T_CreateSeqStmt,
    T_AlterSeqStmt,
    T_VariableSetStmt,
    T_VariableShowStmt,
    T_DiscardStmt,
    T_CreateTrigStmt,
    T_CreatePLangStmt,
    T_CreateRoleStmt,
    T_AlterRoleStmt,
    T_DropRoleStmt,
    T_LockStmt,
    T_ConstraintsSetStmt,
    T_ReindexStmt,
    T_CheckPointStmt,
    T_CreateSchemaStmt,
    T_AlterDatabaseStmt,
    T_AlterDatabaseSetStmt,
    T_AlterRoleSetStmt,
    T_CreateConversionStmt,
    T_CreateCastStmt,
    T_CreateOpClassStmt,
    T_CreateOpFamilyStmt,
    T_AlterOpFamilyStmt,
    T_PrepareStmt,
    T_ExecuteStmt,
    T_DeallocateStmt,
    T_DeclareCursorStmt,
    T_CreateTableSpaceStmt,
    T_DropTableSpaceStmt,
    T_AlterObjectSchemaStmt,
    T_AlterOwnerStmt,
    T_DropOwnedStmt,
    T_ReassignOwnedStmt,
    T_CompositeTypeStmt,
    T_CreateEnumStmt,
    T_CreateRangeStmt,
    T_AlterEnumStmt,
    T_AlterTSDictionaryStmt,
    T_AlterTSConfigurationStmt,
    T_CreateFdwStmt,
    T_AlterFdwStmt,
    T_CreateForeignServerStmt,
    T_AlterForeignServerStmt,
    T_CreateUserMappingStmt,
    T_AlterUserMappingStmt,
    T_DropUserMappingStmt,
    T_AlterTableSpaceOptionsStmt,
    T_AlterTableMoveAllStmt,
    T_SecLabelStmt,
    T_CreateForeignTableStmt,
    T_ImportForeignSchemaStmt,
    T_CreateExtensionStmt,
    T_AlterExtensionStmt,
    T_AlterExtensionContentsStmt,
    T_CreateEventTrigStmt,
    T_AlterEventTrigStmt,
    T_RefreshMatViewStmt,
    T_ReplicaIdentityStmt,
    T_AlterSystemStmt,
    T_CreatePolicyStmt,
    T_AlterPolicyStmt,
    T_A_Expr = 900,
    T_ColumnRef,
    T_ParamRef,
    T_A_Const,
    T_FuncCall,
    T_A_Star,
    T_A_Indices,
    T_A_Indirection,
    T_A_ArrayExpr,
    T_ResTarget,
    T_MultiAssignRef,
    T_TypeCast,
    T_CollateClause,
    T_SortBy,
    T_WindowDef,
    T_RangeSubselect,
    T_RangeFunction,
    T_TypeName,
    T_ColumnDef,
    T_IndexElem,
    T_Constraint,
    T_DefElem,
    T_RangeTblEntry,
    T_RangeTblFunction,
    T_WithCheckOption,
    T_SortGroupClause,
    T_WindowClause,
    T_PrivGrantee,
    T_FuncWithArgs,
    T_AccessPriv,
    T_CreateOpClassItem,
    T_TableLikeClause,
    T_FunctionParameter,
    T_LockingClause,
    T_RowMarkClause,
    T_XmlSerialize,
    T_WithClause,
    T_CommonTableExpr,
    T_IdentifySystemCmd,
    T_BaseBackupCmd,
    T_CreateReplicationSlotCmd,
    T_DropReplicationSlotCmd,
    T_StartReplicationCmd,
    T_TimeLineHistoryCmd,
    T_TriggerData = 950,
    T_EventTriggerData,
    T_ReturnSetInfo,
    T_WindowObjectData,
    T_TIDBitmap,
    T_InlineCodeBlock,
    T_FdwRoutine
}

#[repr(C)]
pub struct Node {
    ty: NodeTag
}

#[repr(C)]
pub struct FmgrInfo {
    fn_addr: PGFunction,
    fn_oid: c_void,
    fn_nargs: u16,
    fn_strict: bool,
    fn_retset: bool,
    fn_stats: u8,
    fn_extra: *mut c_void,
    fn_mcxt: c_void,
    fn_expr: fmNodePtr
}

#[repr(C)]
pub struct FunctionCallInfoData {
    fl_info: *mut c_void,
    context: fmNodePtr,
    result_info: fmNodePtr,
    fn_collation: c_void,
    is_null: bool,
    nargs: u16,
    arg: [Datum, ..FUNC_MAX_ARGS as uint],
    argnull: [bool, ..FUNC_MAX_ARGS as uint]
}

pub struct FunctionCallInfo<'a> {
    ptr: *mut FunctionCallInfoData,
    marker: InvariantLifetime<'a>
}

/// A wrapper around a Postgres `Datum`. A datum is simply
/// a pointer-sized unsigned integer that acts like
/// a pointer.
pub struct Datum {
    val: uint
}

impl Datum {
    pub fn new_str(value: &str) -> Datum {
        // We need to allocate our string onto the heap
        // and with the custom `palloc` allocator. `palloc`
        // allocates memory into contexts such that they
        // can simply drop a while context without incurring
        // any memory leaks (i.e., some extension forgetting to
        // free their memory).
        // let mut mem = unsafe { pg_malloc(value.len() as size_t) };
        Datum {
            val: 0
        }
    }
}

pub struct DatumPtr<'a> {
    ptr: UnsafeCell<Datum>,
    marker: InvariantLifetime<'a>
}

/// The magic metadata that Postgres will ready by calling
/// the `Pg_magic_func` which returns a pointer to
/// this record.
#[repr(C)]
pub struct Pg_magic_struct {
    pub len: c_int,
    pub version: c_int,
    pub funcmaxargs: c_int,
    pub indexmaxkeys: c_int,
    pub nameddatalen: c_int,
    pub float4byval: c_int,
    pub float8byval: c_int
}


