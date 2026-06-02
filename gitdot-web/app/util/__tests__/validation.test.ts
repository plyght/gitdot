import { validateRedirectPath } from "../validation";

describe("validateRedirectPath", () => {
  it("allows same-origin relative paths", () => {
    expect(validateRedirectPath("/")).toBe("/");
    expect(validateRedirectPath("/oauth/device")).toBe("/oauth/device");
    expect(validateRedirectPath("/some-user/their-repo")).toBe(
      "/some-user/their-repo",
    );
    expect(validateRedirectPath("/settings?tab=profile#section")).toBe(
      "/settings?tab=profile#section",
    );
  });

  it("falls back when no path is provided", () => {
    expect(validateRedirectPath(undefined)).toBe("/");
    expect(validateRedirectPath(null)).toBe("/");
    expect(validateRedirectPath("")).toBe("/");
  });

  it("uses the supplied fallback", () => {
    expect(validateRedirectPath(undefined, "/home")).toBe("/home");
    expect(validateRedirectPath("https://evil.example", "/home")).toBe("/home");
  });

  it("rejects absolute URLs", () => {
    expect(validateRedirectPath("https://evil.example/phish")).toBe("/");
    expect(validateRedirectPath("http://evil.example")).toBe("/");
    expect(validateRedirectPath("javascript:alert(1)")).toBe("/");
  });

  it("rejects protocol-relative and backslash bypasses", () => {
    expect(validateRedirectPath("//evil.example")).toBe("/");
    expect(validateRedirectPath("/\\evil.example")).toBe("/");
    expect(validateRedirectPath("/\\/evil.example")).toBe("/");
  });

  it("rejects values that don't start with a slash", () => {
    expect(validateRedirectPath("evil.example")).toBe("/");
    expect(validateRedirectPath("../../etc/passwd")).toBe("/");
  });

  it("rejects whitespace / CRLF header-injection attempts", () => {
    expect(validateRedirectPath("/foo\r\nLocation: https://evil")).toBe("/");
    expect(validateRedirectPath("/foo\tbar")).toBe("/");
    expect(validateRedirectPath("/foo bar")).toBe("/");
  });
});
