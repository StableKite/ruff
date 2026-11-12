from ctypes import Structure, c_long


class Output(Structure):
    _fields_ = [("value", c_long)]

    # ctypes invokes this method when converting native output parameters.
    def __ctypes_from_outparam__(self):
        return self.value


class IntegerOutput(c_long):
    def __ctypes_from_outparam__(self):
        return self.value


class InheritedOutput(Output):
    def __ctypes_from_outparam__(self):
        return self.value


class InvalidNames:
    def __ctypes_from_outparam_(self):
        pass

    def _ctypes_from_outparam__(self):
        pass

    def ___ctypes_from_outparam__(self):
        pass

    def _ctypes_from_outparam_(self):
        pass

    def __ctypes_from_outparams__(self):
        pass

    def __ctypes_from_inparam__(self):
        pass

    def __not_a_native_protocol__(self):
        pass


def convert_output(value):
    # No Python operator or built-in function replaces this conversion hook.
    return value.__ctypes_from_outparam__()


def stringify(value):
    return value.__str__()
