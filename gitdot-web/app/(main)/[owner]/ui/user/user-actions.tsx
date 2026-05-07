export function UserActions() {
  const actions = ["new repo", "new org", "profile", "settings"];

  return (
    <div className="flex flex-col items-end">
      <p className="font-semibold text-sm mb-0.5">actions</p>
      {actions.map((action) => (
        <button
          key={action}
          type="button"
          className="text-xs underline decoration-transparent hover:decoration-current transition-colors duration-200 cursor-pointer"
        >
          {action}
        </button>
      ))}
    </div>
  );
}
