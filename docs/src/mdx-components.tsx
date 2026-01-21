import defaultMdxComponents from 'fumadocs-ui/mdx';
import type { MDXComponents } from 'mdx/types';
import { Steps as FumadocsSteps, Step as FumadocsStep } from 'fumadocs-ui/components/steps';

function Steps({ children }: { children: React.ReactNode }) {
  return <FumadocsSteps>{children}</FumadocsSteps>;
}

function Step({ title, children }: { title?: string; children: React.ReactNode }) {
  return (
    <FumadocsStep>
      {title && <div className="font-semibold">{title}</div>}
      {children}
    </FumadocsStep>
  );
}

export function getMDXComponents(components?: MDXComponents): MDXComponents {
  return {
    ...defaultMdxComponents,
    Steps,
    Step,
    ...components,
  };
}
