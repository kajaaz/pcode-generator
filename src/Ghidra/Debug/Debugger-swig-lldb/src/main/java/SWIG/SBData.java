/* ###
 * IP: Apache License 2.0 with LLVM Exceptions
 */
/* ----------------------------------------------------------------------------
 * This file was automatically generated by SWIG (http://www.swig.org).
 * Version 4.0.1
 *
 * Do not make changes to this file unless you know what you are doing--modify
 * the SWIG interface file instead.
 * ----------------------------------------------------------------------------- */

package SWIG;

public class SBData {
  private transient long swigCPtr;
  protected transient boolean swigCMemOwn;

  protected SBData(long cPtr, boolean cMemoryOwn) {
    swigCMemOwn = cMemoryOwn;
    swigCPtr = cPtr;
  }

  protected static long getCPtr(SBData obj) {
    return (obj == null) ? 0 : obj.swigCPtr;
  }

  @SuppressWarnings("deprecation")
  protected void finalize() {
    delete();
  }

  public synchronized void delete() {
    if (swigCPtr != 0) {
      if (swigCMemOwn) {
        swigCMemOwn = false;
        lldbJNI.delete_SBData(swigCPtr);
      }
      swigCPtr = 0;
    }
  }

  public SBData() {
    this(lldbJNI.new_SBData__SWIG_0(), true);
  }

  public SBData(SBData rhs) {
    this(lldbJNI.new_SBData__SWIG_1(SBData.getCPtr(rhs), rhs), true);
  }

  public short GetAddressByteSize() {
    return lldbJNI.SBData_GetAddressByteSize(swigCPtr, this);
  }

  public void SetAddressByteSize(short addr_byte_size) {
    lldbJNI.SBData_SetAddressByteSize(swigCPtr, this, addr_byte_size);
  }

  public void Clear() {
    lldbJNI.SBData_Clear(swigCPtr, this);
  }

  public boolean IsValid() {
    return lldbJNI.SBData_IsValid(swigCPtr, this);
  }

  public long GetByteSize() {
    return lldbJNI.SBData_GetByteSize(swigCPtr, this);
  }

  public ByteOrder GetByteOrder() {
    return ByteOrder.swigToEnum(lldbJNI.SBData_GetByteOrder(swigCPtr, this));
  }

  public void SetByteOrder(ByteOrder endian) {
    lldbJNI.SBData_SetByteOrder(swigCPtr, this, endian.swigValue());
  }

  public float GetFloat(SBError error, java.math.BigInteger offset) {
    return lldbJNI.SBData_GetFloat(swigCPtr, this, SBError.getCPtr(error), error, offset);
  }

  public double GetDouble(SBError error, java.math.BigInteger offset) {
    return lldbJNI.SBData_GetDouble(swigCPtr, this, SBError.getCPtr(error), error, offset);
  }

  public SWIGTYPE_p_long_double GetLongDouble(SBError error, java.math.BigInteger offset) {
    return new SWIGTYPE_p_long_double(lldbJNI.SBData_GetLongDouble(swigCPtr, this, SBError.getCPtr(error), error, offset), true);
  }

  public java.math.BigInteger GetAddress(SBError error, java.math.BigInteger offset) {
    return lldbJNI.SBData_GetAddress(swigCPtr, this, SBError.getCPtr(error), error, offset);
  }

  public short GetUnsignedInt8(SBError error, java.math.BigInteger offset) {
    return lldbJNI.SBData_GetUnsignedInt8(swigCPtr, this, SBError.getCPtr(error), error, offset);
  }

  public int GetUnsignedInt16(SBError error, java.math.BigInteger offset) {
    return lldbJNI.SBData_GetUnsignedInt16(swigCPtr, this, SBError.getCPtr(error), error, offset);
  }

  public long GetUnsignedInt32(SBError error, java.math.BigInteger offset) {
    return lldbJNI.SBData_GetUnsignedInt32(swigCPtr, this, SBError.getCPtr(error), error, offset);
  }

  public java.math.BigInteger GetUnsignedInt64(SBError error, java.math.BigInteger offset) {
    return lldbJNI.SBData_GetUnsignedInt64(swigCPtr, this, SBError.getCPtr(error), error, offset);
  }

  public byte GetSignedInt8(SBError error, java.math.BigInteger offset) {
    return lldbJNI.SBData_GetSignedInt8(swigCPtr, this, SBError.getCPtr(error), error, offset);
  }

  public short GetSignedInt16(SBError error, java.math.BigInteger offset) {
    return lldbJNI.SBData_GetSignedInt16(swigCPtr, this, SBError.getCPtr(error), error, offset);
  }

  public int GetSignedInt32(SBError error, java.math.BigInteger offset) {
    return lldbJNI.SBData_GetSignedInt32(swigCPtr, this, SBError.getCPtr(error), error, offset);
  }

  public long GetSignedInt64(SBError error, java.math.BigInteger offset) {
    return lldbJNI.SBData_GetSignedInt64(swigCPtr, this, SBError.getCPtr(error), error, offset);
  }

  public String GetString(SBError error, java.math.BigInteger offset) {
    return lldbJNI.SBData_GetString(swigCPtr, this, SBError.getCPtr(error), error, offset);
  }

  public boolean GetDescription(SBStream description, java.math.BigInteger base_addr) {
    return lldbJNI.SBData_GetDescription(swigCPtr, this, SBStream.getCPtr(description), description, base_addr);
  }

  public long ReadRawData(SBError error, java.math.BigInteger offset, SWIGTYPE_p_void buf, long size) {
    return lldbJNI.SBData_ReadRawData(swigCPtr, this, SBError.getCPtr(error), error, offset, SWIGTYPE_p_void.getCPtr(buf), size);
  }

  public void SetData(SBError error, SWIGTYPE_p_void buf, long size, ByteOrder endian, short addr_size) {
    lldbJNI.SBData_SetData(swigCPtr, this, SBError.getCPtr(error), error, SWIGTYPE_p_void.getCPtr(buf), size, endian.swigValue(), addr_size);
  }

  public void SetDataWithOwnership(SBError error, SWIGTYPE_p_void buf, long size, ByteOrder endian, short addr_size) {
    lldbJNI.SBData_SetDataWithOwnership(swigCPtr, this, SBError.getCPtr(error), error, SWIGTYPE_p_void.getCPtr(buf), size, endian.swigValue(), addr_size);
  }

  public boolean Append(SBData rhs) {
    return lldbJNI.SBData_Append(swigCPtr, this, SBData.getCPtr(rhs), rhs);
  }

  public static SBData CreateDataFromCString(ByteOrder endian, long addr_byte_size, String data) {
    return new SBData(lldbJNI.SBData_CreateDataFromCString(endian.swigValue(), addr_byte_size, data), true);
  }

  public static SBData CreateDataFromUInt64Array(ByteOrder endian, long addr_byte_size, SWIGTYPE_p_unsigned_long_long array, long array_len) {
    return new SBData(lldbJNI.SBData_CreateDataFromUInt64Array(endian.swigValue(), addr_byte_size, SWIGTYPE_p_unsigned_long_long.getCPtr(array), array_len), true);
  }

  public static SBData CreateDataFromUInt32Array(ByteOrder endian, long addr_byte_size, SWIGTYPE_p_unsigned_int array, long array_len) {
    return new SBData(lldbJNI.SBData_CreateDataFromUInt32Array(endian.swigValue(), addr_byte_size, SWIGTYPE_p_unsigned_int.getCPtr(array), array_len), true);
  }

  public static SBData CreateDataFromSInt64Array(ByteOrder endian, long addr_byte_size, SWIGTYPE_p_long_long array, long array_len) {
    return new SBData(lldbJNI.SBData_CreateDataFromSInt64Array(endian.swigValue(), addr_byte_size, SWIGTYPE_p_long_long.getCPtr(array), array_len), true);
  }

  public static SBData CreateDataFromSInt32Array(ByteOrder endian, long addr_byte_size, SWIGTYPE_p_int array, long array_len) {
    return new SBData(lldbJNI.SBData_CreateDataFromSInt32Array(endian.swigValue(), addr_byte_size, SWIGTYPE_p_int.getCPtr(array), array_len), true);
  }

  public static SBData CreateDataFromDoubleArray(ByteOrder endian, long addr_byte_size, SWIGTYPE_p_double array, long array_len) {
    return new SBData(lldbJNI.SBData_CreateDataFromDoubleArray(endian.swigValue(), addr_byte_size, SWIGTYPE_p_double.getCPtr(array), array_len), true);
  }

  public boolean SetDataFromCString(String data) {
    return lldbJNI.SBData_SetDataFromCString(swigCPtr, this, data);
  }

  public boolean SetDataFromUInt64Array(SWIGTYPE_p_unsigned_long_long array, long array_len) {
    return lldbJNI.SBData_SetDataFromUInt64Array(swigCPtr, this, SWIGTYPE_p_unsigned_long_long.getCPtr(array), array_len);
  }

  public boolean SetDataFromUInt32Array(SWIGTYPE_p_unsigned_int array, long array_len) {
    return lldbJNI.SBData_SetDataFromUInt32Array(swigCPtr, this, SWIGTYPE_p_unsigned_int.getCPtr(array), array_len);
  }

  public boolean SetDataFromSInt64Array(SWIGTYPE_p_long_long array, long array_len) {
    return lldbJNI.SBData_SetDataFromSInt64Array(swigCPtr, this, SWIGTYPE_p_long_long.getCPtr(array), array_len);
  }

  public boolean SetDataFromSInt32Array(SWIGTYPE_p_int array, long array_len) {
    return lldbJNI.SBData_SetDataFromSInt32Array(swigCPtr, this, SWIGTYPE_p_int.getCPtr(array), array_len);
  }

  public boolean SetDataFromDoubleArray(SWIGTYPE_p_double array, long array_len) {
    return lldbJNI.SBData_SetDataFromDoubleArray(swigCPtr, this, SWIGTYPE_p_double.getCPtr(array), array_len);
  }

  public String __repr__() {
    return lldbJNI.SBData___repr__(swigCPtr, this);
  }

}