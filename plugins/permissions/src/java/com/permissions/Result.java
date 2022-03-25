package java.com.permissions;

public class Result {
    private boolean ok;
    private String value;

    public Result(boolean is_ok, String value) {
        this.ok = is_ok;
        this.value = value;
    }

    public boolean isOk() {
        return this.ok;
    }

    public boolean isError() {
        return !this.ok;
    }

    public String getValue() {
        return this.ok ? this.value : null;
    }

    public String getError() {
        return this.ok ? null : this.value;
    }
}