import type { OrganizationResource } from "gitdot-api";

export function OrgLinks({ org }: { org: OrganizationResource }) {
  const links = (org as OrganizationResource & { links?: string[] }).links;
  if (!links?.length) return null;

  return (
    <div className="flex flex-col items-end">
      <p className="font-semibold text-sm mb-0.5">links</p>
      {links.map((link, i) => (
        <a
          key={i}
          href={/^https?:\/\//.test(link) ? link : `https://${link}`}
          target="_blank"
          rel="noopener noreferrer"
          className="text-xs underline decoration-transparent hover:decoration-current transition-colors duration-200"
        >
          {link.replace(/^https?:\/\//, "")}
        </a>
      ))}
    </div>
  );
}
