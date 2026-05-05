import ConnectSlackForm from "../../ui/connect-slack-form";

export default async function Page({
  searchParams,
}: {
  searchParams: Promise<{ state?: string }>;
}) {
  const { state } = await searchParams;

  return (
    <div className="max-w-3xl mx-auto flex gap-4 items-start justify-center h-screen pt-[45vh]">
      <ConnectSlackForm state={state} />
    </div>
  );
}
