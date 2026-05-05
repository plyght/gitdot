import LoginForm from "../ui/login-form";

export default async function Page({
  searchParams,
}: {
  searchParams: Promise<{ redirect?: string }>;
}) {
  const { redirect } = await searchParams;

  return (
    <div className="max-w-3xl mx-auto flex items-start justify-center h-screen pt-[45vh]">
      <LoginForm redirect={redirect || "/home"} />
    </div>
  );
}
